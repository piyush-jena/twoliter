#!/usr/bin/env bash
set -euo pipefail
source "$(dirname "$0")/../ab-boot-helper"
SYSTEMD_BOOT_AB=yes
UEFI_SECURE_BOOT=yes
IN_PLACE_UPDATES=yes
UKI_IMAGE=no
STANDALONE_IMAGE=no
ENCRYPTED_STORAGE=no
ab_boot_validate
for flag in UEFI_SECURE_BOOT IN_PLACE_UPDATES; do
  (declare "${flag}=no"; ! ab_boot_validate)
done
for flag in UKI_IMAGE STANDALONE_IMAGE ENCRYPTED_STORAGE; do
  (declare "${flag}=yes"; ! ab_boot_validate)
done
SYSTEMD_BOOT_AB=no
UEFI_SECURE_BOOT=no
IN_PLACE_UPDATES=no
ab_boot_validate
actual="$(ab_boot_verity 'root,,,ro,0 8 verity 1 PARTUUID=$boot_uuid/PARTNROFF=1 PARTUUID=$boot_uuid/PARTNROFF=2 4096 4096')"
[[ "${actual}" == 'root,,,ro,0 8 verity 1 PARTUUID=@ROOT@ PARTUUID=@HASH@ 4096 4096' ]]
echo 'PASS authenticated GPT feature constraints and typed verity substitutions'
