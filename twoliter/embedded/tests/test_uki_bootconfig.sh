#!/usr/bin/env bash
#
# Test the UKI bootconfig cmdline helpers in `imghelper`:
# `render_bootconfig_cmdline`, `uki_bootconfig_kernel_cmdline`, and
# `uki_bootconfig_init_cmdline`.
#
# These DERIVE the UKI cmdline tokens from the on-image boot-config.d drop-in
# snippets (/boot/boot-config.d/*.conf) -- the same files the GRUB build
# concatenates + sorts into bootconfig.data -- rather than hard-coding them.
# This is the single source of truth (spec T08 / C5 / AC-9): both boot paths
# consume the same snippet files. The rendering mirrors lib/bootconfig.c
# `xbc_snprint_cmdline`: `kernel.*` values are prepended to the front of the
# cmdline and `init.*` values inserted after `--`, in each case with the
# section prefix stripped and `key = value` normalised to `key=value`, in the
# kernel's xbc tree order (= the sorted order of the concatenated lines).
#
# Run from the repo root:
#   bash twoliter/embedded/tests/test_uki_bootconfig.sh

set -eu -o pipefail

# The renderer's token order comes from `sort`. Pin the C locale so the byte
# ordering asserted below is deterministic and matches the documented xbc tree
# order (uppercase 'S' before lowercase 'f'/'m').
export LC_ALL=C

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IMGHELPER="${SCRIPT_DIR}/../imghelper"

if [[ ! -f "${IMGHELPER}" ]]; then
  echo "test_uki_bootconfig: imghelper not found at ${IMGHELPER}" >&2
  exit 1
fi

# imghelper's top-level `${VAR:?}` expansions require these to be set before
# it can be sourced, even though this test only exercises the bootconfig
# renderer helpers.
IMAGE_NAME=x VARIANT=x ARCH=x86_64 VERSION_ID=x BUILD_ID=x
export IMAGE_NAME VARIANT ARCH VERSION_ID BUILD_ID

# shellcheck source=../imghelper
. "${IMGHELPER}"

pass_count=0
fail_count=0

pass() {
  pass_count=$((pass_count + 1))
  echo "  ok: $1"
}

fail() {
  fail_count=$((fail_count + 1))
  echo "  FAIL: $1" >&2
}

assert_eq() {
  local actual expected name
  actual="$1"
  expected="$2"
  name="$3"
  if [[ "${actual}" == "${expected}" ]]; then
    pass "${name}"
  else
    fail "${name}: expected '${expected}' got '${actual}'"
  fi
}

# Assert that a space-separated token list does NOT contain any of the tokens.
assert_absent_tokens() {
  local actual name token
  actual="$1"
  name="$2"
  shift 2
  for token in "$@"; do
    if [[ " ${actual} " == *" ${token} "* ]]; then
      fail "${name}: token '${token}' must be absent from '${actual}'"
      return
    fi
  done
  pass "${name}"
}

# Build a boot-config.d directory populated with the real snippet filenames and
# contents shipped by the kits, echoing the directory path. The systemd and aws
# snippets are always present; the FIPS snippet is added only when $1 == fips.
# Whitespace around `=` is intentionally mixed to prove xbc-style normalisation.
make_bootconfig_dir() {
  local fips dir
  fips="${1:-nofips}"
  dir="$(mktemp -d)"
  # kernel kit, aws platform (05-aws.conf): the canonical home of module_blacklist.
  printf 'kernel.module_blacklist = i8042\n' >"${dir}/05-aws.conf"
  # systemd-257 (20-*, 21-*): note one uses spaces around '=', one does not.
  printf 'kernel.SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25\n' >"${dir}/20-mount-rate-limit-burst.conf"
  printf 'kernel.SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1\n' >"${dir}/21-cgroup-enable-legacy-force.conf"
  if [[ "${fips}" == fips ]]; then
    # release (10-fips.conf): a kernel.* and an init.* token in one file.
    printf 'kernel.fips = 1\ninit.systemd.unit = fipscheck.target\n' >"${dir}/10-fips.conf"
  fi
  echo "${dir}"
}

echo "Test 1: FIPS image (10-fips.conf present) kernel/init sections"
dir="$(make_bootconfig_dir fips)"
kout="$(uki_bootconfig_kernel_cmdline "${dir}")"
iout="$(uki_bootconfig_init_cmdline "${dir}")"
# Kernel section renders in xbc tree order = sorted concatenated lines:
# SYSTEMD_CGROUP, SYSTEMD_DEFAULT, fips (0x66), module_blacklist (0x6d).
assert_eq "${kout}" \
  "SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 fips=1 module_blacklist=i8042" \
  "fips image: kernel section tokens, order, and key=value normalisation"
# init.* tokens live after `--` with the `init.` prefix stripped.
assert_eq "${iout}" "systemd.unit=fipscheck.target" \
  "fips image: init section token (init. prefix stripped)"
rm -rf "${dir}"

echo
echo "Test 2: non-FIPS image (no 10-fips.conf) kernel/init sections"
dir="$(make_bootconfig_dir nofips)"
kout="$(uki_bootconfig_kernel_cmdline "${dir}")"
iout="$(uki_bootconfig_init_cmdline "${dir}")"
assert_eq "${kout}" \
  "SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 module_blacklist=i8042" \
  "non-fips image: kernel section omits fips tokens"
assert_eq "${iout}" "" "non-fips image: init section is empty"
assert_absent_tokens "${kout} ${iout}" "non-fips image: FIPS tokens absent" \
  "fips=1" "systemd.unit=fipscheck.target"
rm -rf "${dir}"

echo
echo "Test 3: module_blacklist is sourced from the snippet, not hard-coded"
# It must appear in the DERIVED kernel section (single source of truth = the
# 05-aws.conf bootconfig snippet), proving it is no longer hard-coded away.
dir="$(make_bootconfig_dir nofips)"
kout="$(uki_bootconfig_kernel_cmdline "${dir}")"
if [[ " ${kout} " == *" module_blacklist=i8042 "* ]]; then
  pass "module_blacklist=i8042 is derived from the boot-config.d snippet"
else
  fail "module_blacklist=i8042 missing from derived kernel section: '${kout}'"
fi
rm -rf "${dir}"

echo
echo "Test 4: absent or empty boot-config.d yields empty output"
assert_eq "$(uki_bootconfig_kernel_cmdline "/nonexistent-$$")" "" \
  "absent directory: kernel section empty"
empty_dir="$(mktemp -d)"
assert_eq "$(uki_bootconfig_kernel_cmdline "${empty_dir}")" "" \
  "empty directory: kernel section empty"
assert_eq "$(uki_bootconfig_init_cmdline "${empty_dir}")" "" \
  "empty directory: init section empty"
rm -rf "${empty_dir}"

echo
echo "Test 5: comments and blank lines in snippets are ignored"
dir="$(mktemp -d)"
printf '# a comment\n\nkernel.foo=bar\n   \ninit.baz = qux\n' >"${dir}/99-misc.conf"
assert_eq "$(uki_bootconfig_kernel_cmdline "${dir}")" "foo=bar" \
  "comments/blanks ignored (kernel)"
assert_eq "$(uki_bootconfig_init_cmdline "${dir}")" "baz=qux" \
  "comments/blanks ignored (init)"
rm -rf "${dir}"

echo
echo "Test 6: non-UKI images never call the UKI bootconfig helpers"
# rpm2img only calls the uki_bootconfig_* helpers inside the `UKI_IMAGE == yes`
# branch; a non-UKI build never invokes them. We assert the source-level guard
# directly, since rpm2img itself is not practical to run end-to-end in a unit
# test (it performs real partitioning, RPM installs, and image assembly first).
RPM2IMG="${SCRIPT_DIR}/../rpm2img"
if [[ ! -f "${RPM2IMG}" ]]; then
  echo "test_uki_bootconfig: rpm2img not found at ${RPM2IMG}" >&2
  exit 1
fi
if grep -qE '^\s*if \[\[ "\$\{UKI_IMAGE\}" == "yes" \]\]; then\s*$' "${RPM2IMG}"; then
  # Extract the UKI-image conditional block and confirm the uki_bootconfig_*
  # calls live inside it.
  uki_block="$(awk '/if \[\[ "\$\{UKI_IMAGE\}" == "yes" \]\]; then/,/^fi$/' "${RPM2IMG}")"
  if [[ "${uki_block}" == *"uki_bootconfig_kernel_cmdline"* \
     && "${uki_block}" == *"uki_bootconfig_init_cmdline"* ]]; then
    pass "uki_bootconfig_* helpers are only invoked inside the UKI_IMAGE=yes branch"
  else
    fail "uki_bootconfig_* calls not found inside the UKI_IMAGE=yes branch"
  fi
else
  fail "could not locate the UKI_IMAGE=yes conditional in rpm2img"
fi

echo
echo "Results: ${pass_count} passed, ${fail_count} failed"
if [[ "${fail_count}" -gt 0 ]]; then
  exit 1
fi
