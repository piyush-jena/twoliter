#!/usr/bin/env bash
# Build the disk fixtures for pcrsys's end-to-end tests with the tools rpm2img uses.
# Usage: make-disks.sh INPUT_DIR OUTPUT_DIR
#
# INPUT_DIR holds bootaa64.efi, grubaa64.efi, vmlinuz, grub.cfg, bootconfig.data,
# BOOTX64.EFI (a UKI), and x86_64/ with bootx64.efi, grubx64.efi and vmlinuz.
# Identifiers and timestamps are pinned, so the output is reproducible.
set -euo pipefail

input="$(realpath "${1:?input dir}")"
output="$(realpath "${2:?output dir}")"
work="$(mktemp -d)"
trap 'rm -rf "${work}"' EXIT

export SOURCE_DATE_EPOCH=946684800 # 2000-01-01, which FAT can represent
export E2FSPROGS_FAKE_TIME="${SOURCE_DATE_EPOCH}"
export MTOOLS_SKIP_CHECK=1
touch -d "@${SOURCE_DATE_EPOCH}" "${input}"/* "${input}"/x86_64/*

readonly BIOS_TYPE=ef02
readonly EFI_TYPE=C12A7328-F81F-11D2-BA4B-00A0C93EC93B
readonly BOOT_TYPE=6b636168-7420-6568-2070-6c616e657421
readonly ROOT_TYPE=5526016a-1a97-4ea4-b39a-b7c8c6ca4502
readonly PRIVATE_TYPE=440408bb-eb0b-4328-a6e5-a29038fad706

# A FAT ESP holding each named input file in /EFI/BOOT.
make_esp() {
  local image="$1" size_mib="$2"
  shift 2
  rm -f "${image}"
  mformat -i "${image}" -C -T "$((size_mib * 2048))" -h 64 -s 32 -N 12345678 -v EFI ::
  mmd -i "${image}" ::/EFI ::/EFI/BOOT
  local file
  for file in "$@"; do
    mcopy -m -i "${image}" "${input}/${file}" "::/EFI/BOOT/${file##*/}"
  done
}

# An ext4 filesystem from the directory DIR, using rpm2img's flags for its role.
make_ext4() {
  local image="$1" size_mib="$2" dir="$3" uuid="$4"
  shift 4
  rm -f "${image}"
  mkfs.ext4 -q -F -U "${uuid}" -E "hash_seed=${uuid},root_owner=0:0" "$@" -d "${dir}" \
    "${image}" "${size_mib}M"
}

# Assemble DISK from "number:label:type:guid:size_mib:attributes:image" partition specs.
make_disk() {
  local disk="$1"
  shift
  local spec number label type guid size attributes image
  local start=1 total=1 args=()
  for spec in "$@"; do
    IFS=: read -r number label type guid size attributes image <<<"${spec}"
    args+=(-n "${number}:${start}M:$(((start + size) * 2048 - 1))" -c "${number}:${label}")
    args+=(-t "${number}:${type}" -u "${number}:${guid}")
    for attribute in ${attributes//,/ }; do
      args+=(-A "${number}:set:${attribute}")
    done
    start=$((start + size))
  done
  total=$((start + 1))
  rm -f "${disk}"
  truncate -s "${total}M" "${disk}"
  sgdisk --clear -U 7e1a8c3e-0f5d-4a3b-9c2d-5e6f7a8b9c0d "${args[@]}" "${disk}" >/dev/null
  start=1
  for spec in "$@"; do
    IFS=: read -r number label type guid size attributes image <<<"${spec}"
    if [[ -n "${image}" ]]; then
      dd if="${image}" of="${disk}" bs=1M seek="${start}" conv=notrunc status=none
    fi
    start=$((start + size))
  done
  sgdisk -v "${disk}" >/dev/null
  gzip -n -9 -c "${disk}" >"${output}/$(basename "${disk}").gz"
}

mkdir -p "${work}/boot/grub" "${work}/boot-x86_64/grub" "${work}/private" "${work}/empty"
cp "${input}/vmlinuz" "${work}/boot/vmlinuz"
cp "${input}/x86_64/vmlinuz" "${work}/boot-x86_64/vmlinuz"
cp "${input}/grub.cfg" "${work}/boot/grub/grub.cfg"
cp "${input}/grub.cfg" "${work}/boot-x86_64/grub/grub.cfg"
cp "${input}/bootconfig.data" "${work}/private/bootconfig.data"
touch -d "@${SOURCE_DATE_EPOCH}" "${work}/boot" "${work}/boot/grub" "${work}"/boot/*/* \
  "${work}"/boot/vmlinuz "${work}/boot-x86_64" "${work}/boot-x86_64/grub" \
  "${work}"/boot-x86_64/*/* "${work}"/boot-x86_64/vmlinuz "${work}/private" \
  "${work}"/private/* "${work}/empty"

make_esp "${work}/esp-grub.img" 2 bootaa64.efi grubaa64.efi
make_esp "${work}/esp-grub-x86_64.img" 2 x86_64/bootx64.efi x86_64/grubx64.efi
make_esp "${work}/esp-uki.img" 4 BOOTX64.EFI
make_ext4 "${work}/boot.img" 4 "${work}/boot" 3c7b8f0e-6a2d-4e1f-8b9a-0d1c2e3f4a5b -O ^has_journal
make_ext4 "${work}/boot-x86_64.img" 4 "${work}/boot-x86_64" 3c7b8f0e-6a2d-4e1f-8b9a-0d1c2e3f4a5b \
  -O ^has_journal
make_ext4 "${work}/private.img" 2 "${work}/private" 9a8b7c6d-5e4f-4a3b-8c2d-1e0f9a8b7c6d \
  -b 4096 -i 4096 -I 256
make_ext4 "${work}/private-empty.img" 2 "${work}/empty" 9a8b7c6d-5e4f-4a3b-8c2d-1e0f9a8b7c6d \
  -b 4096 -i 4096 -I 256

bios="1:BIOS-BOOT:${BIOS_TYPE}:0d4f1e2a-3b4c-4d5e-8f60-718293a4b5c6:1::"
esp_grub="2:EFI-SYSTEM:${EFI_TYPE}:1a2b3c4d-5e6f-4a1b-8c2d-3e4f5a6b7c8d:2::${work}/esp-grub.img"
esp_grub_x86_64="${esp_grub%:*}:${work}/esp-grub-x86_64.img"
esp_uki="2:EFI-SYSTEM:${EFI_TYPE}:1a2b3c4d-5e6f-4a1b-8c2d-3e4f5a6b7c8d:4::${work}/esp-uki.img"
boot_a="3:BOTTLEROCKET-BOOT-A:${BOOT_TYPE}:bd06db87-42ee-410f-8092-e7b1130b59bf:4:48,56:${work}/boot.img"
boot_a_x86_64="${boot_a%:*}:${work}/boot-x86_64.img"
root_a="4:BOTTLEROCKET-ROOT-A:${ROOT_TYPE}:2b3c4d5e-6f70-4a1b-9c2d-4e5f6a7b8c9d:1::"
boot_b="5:BOTTLEROCKET-BOOT-B:${BOOT_TYPE}:3c4d5e6f-7081-4b2c-8d3e-5f6a7b8c9dae:4::"
root_b="6:BOTTLEROCKET-ROOT-B:${ROOT_TYPE}:4d5e6f70-8192-4c3d-9e4f-6a7b8c9daebf:1::"
private="7:BOTTLEROCKET-PRIVATE:${PRIVATE_TYPE}:5e6f7081-92a3-4d4e-8f50-7b8c9daebfc0:2::${work}/private.img"
private_uki="5:BOTTLEROCKET-PRIVATE:${PRIVATE_TYPE}:5e6f7081-92a3-4d4e-8f50-7b8c9daebfc0:2::${work}/private-empty.img"

make_disk "${work}/ab.img" "${bios}" "${esp_grub}" "${boot_a}" "${root_a}" "${boot_b}" \
  "${root_b}" "${private}"
make_disk "${work}/single-bank-aarch64.img" "${bios}" "${esp_grub}" "${boot_a}" "${root_a}" \
  "${private/7:/5:}"
make_disk "${work}/single-bank-x86_64.img" "${bios}" "${esp_grub_x86_64}" "${boot_a_x86_64}" \
  "${root_a}" "${private/7:/5:}"
make_disk "${work}/uki.img" "${bios}" "${esp_uki}" "${root_a/4:/3:}" "${private_uki/5:/4:}"
