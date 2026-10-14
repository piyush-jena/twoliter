# Blacklist regression using extracted release tools

Old release: `09380d356886842b8987b602ed2ef7daaefec4f3`. Final release: `830ee97501b05adbf6dc76d13383643e440a6002`.

## mixed-block: old

Input:
```text
kernel { module_blacklist = binfmt_misc }
kernel.module_blacklist = i8042
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/mixed-block/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/mixed-block/old/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel { module_blacklist = binfmt_misc }
kernel.module_blacklist = "i8042"
```

## mixed-block: final

Input:
```text
kernel { module_blacklist = binfmt_misc }
kernel.module_blacklist = i8042
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/mixed-block/final/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/mixed-block/final/bootconfig.in
```

Exit code: 1.

stdout:
```text
```

stderr:
```text
Error: Failed to merge bootconfig module blacklists *

Caused by this error:
  1: Module blacklist must be a single-line assignment

NOTE: Some redundant information has been removed from the lines marked with *. Set SNAFU_RAW_ERROR_MESSAGES=1 to disable this behavior.

```

Resulting input file:
```text
kernel { module_blacklist = binfmt_misc }
kernel.module_blacklist = i8042
```

## canonical: old

Input:
```text
kernel.module_blacklist = i8042
kernel.module_blacklist = binfmt_misc
kernel.module_blacklist = i8042
kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/canonical/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/canonical/old/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount
kernel.module_blacklist = "binfmt_misc,i8042"
```

## canonical: final

Input:
```text
kernel.module_blacklist = i8042
kernel.module_blacklist = binfmt_misc
kernel.module_blacklist = i8042
kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/canonical/final/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/canonical/final/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount
kernel.module_blacklist = "binfmt_misc,i8042"
```

## aliases: old

Input:
```text
kernel.module-blacklist = i8042
kernel.module_blacklist = binfmt_misc
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/aliases/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/aliases/old/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.module_blacklist = "binfmt_misc,i8042"
```

## aliases: final

Input:
```text
kernel.module-blacklist = i8042
kernel.module_blacklist = binfmt_misc
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/aliases/final/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/aliases/final/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.module_blacklist = "binfmt_misc,i8042"
```

## arrays: old

Input:
```text
kernel.module_blacklist = i8042, binfmt_misc
kernel.console = tty0, ttyS0
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/arrays/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/arrays/old/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.console = tty0, ttyS0
kernel.module_blacklist = "binfmt_misc,i8042"
```

## arrays: final

Input:
```text
kernel.module_blacklist = i8042, binfmt_misc
kernel.console = tty0, ttyS0
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/arrays/final/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/arrays/final/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.console = tty0, ttyS0
kernel.module_blacklist = "binfmt_misc,i8042"
```

## plain: old

Input:
```text
kernel.console = tty0, ttyS0
kernel.quiet
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/plain/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/plain/old/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.console = tty0, ttyS0
kernel.quiet
```

## plain: final

Input:
```text
kernel.console = tty0, ttyS0
kernel.quiet
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/plain/final/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/plain/final/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.console = tty0, ttyS0
kernel.quiet
```

## fips: old

Input:
```text
kernel.fips = 1
kernel.module_blacklist = i8042
kernel.module-blacklist = binfmt_misc
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/fips/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/fips/old/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.fips = 1
kernel.module_blacklist = "binfmt_misc,i8042"
```

## fips: final

Input:
```text
kernel.fips = 1
kernel.module_blacklist = i8042
kernel.module-blacklist = binfmt_misc
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/fips/final/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/release-blacklist/fips/final/bootconfig.in
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
```

Resulting input file:
```text
kernel.fips = 1
kernel.module_blacklist = "binfmt_misc,i8042"
```
