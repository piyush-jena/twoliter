#!/usr/bin/env bash
set -euo pipefail
. "$(dirname "$0")/../partyplanner"
for size in 2 4 8; do
  declare -A partsize=() partoff=()
  set_partition_sizes "$size" 20 split yes partsize partoff no yes
  test "${partsize[BOOT-A]}" = 0
  test "${partsize[BOOT-B]}" = 0
  test "${partsize[EFI-B]}" = 0
  test "${partsize[ROOT-A]}" = "${partsize[ROOT-B]}"
  test "${partsize[HASH-A]}" = "${partsize[HASH-B]}"
  test "${partoff[ROOT-A]}" -eq "$((partoff[EFI-A] + partsize[EFI-A]))"
  test "${partoff[HASH-A]}" -eq "$((partoff[ROOT-A] + partsize[ROOT-A]))"
  test "${partoff[ROOT-B]}" -eq "$((partoff[RESERVED-A] + partsize[RESERVED-A]))"
  test "${partoff[PRIVATE]}" -eq "$((partoff[RESERVED-B] + partsize[RESERVED-B]))"
  echo "PASS: ${size} GiB UKI layout has one ESP, disjoint equal root/hash banks, preserved private boundary"
done
