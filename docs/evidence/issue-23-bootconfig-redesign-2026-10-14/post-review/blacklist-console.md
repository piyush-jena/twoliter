# Post-review blacklist regression and output comparison

The old binary is the release used for all ten live boots. The new binary is
a local Rust 1.97.1 build of final source `830ee97501b05adbf6dc76d13383643e440a6002`.

## mixed-block: old

Input:
```text
kernel { module_blacklist = binfmt_misc }
kernel.module_blacklist = i8042
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/mixed-block/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/mixed-block/old/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/mixed-block/old/bootconfig.in
kernel { module_blacklist = binfmt_misc }
kernel.module_blacklist = "i8042"
```

## mixed-block: new

Input:
```text
kernel { module_blacklist = binfmt_misc }
kernel.module_blacklist = i8042
```

```console
$ /tmp/symphony-ukisys-redesign-check/target/debug/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/mixed-block/new/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/mixed-block/new/bootconfig.in
[exit 1]
stdout:

stderr:
Error: Failed to merge bootconfig module blacklists *

Caused by this error:
  1: Module blacklist must be a single-line assignment

NOTE: Some redundant information has been removed from the lines marked with *. Set SNAFU_RAW_ERROR_MESSAGES=1 to disable this behavior.
$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/mixed-block/new/bootconfig.in
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
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/canonical/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/canonical/old/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/canonical/old/bootconfig.in
kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount
kernel.module_blacklist = "binfmt_misc,i8042"
```

## canonical: new

Input:
```text
kernel.module_blacklist = i8042
kernel.module_blacklist = binfmt_misc
kernel.module_blacklist = i8042
kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount
```

```console
$ /tmp/symphony-ukisys-redesign-check/target/debug/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/canonical/new/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/canonical/new/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/canonical/new/bootconfig.in
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
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/aliases/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/aliases/old/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/aliases/old/bootconfig.in
kernel.module_blacklist = "binfmt_misc,i8042"
```

## aliases: new

Input:
```text
kernel.module-blacklist = i8042
kernel.module_blacklist = binfmt_misc
```

```console
$ /tmp/symphony-ukisys-redesign-check/target/debug/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/aliases/new/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/aliases/new/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/aliases/new/bootconfig.in
kernel.module_blacklist = "binfmt_misc,i8042"
```

## arrays: old

Input:
```text
kernel.module_blacklist = i8042, binfmt_misc
kernel.console = tty0, ttyS0
```

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/arrays/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/arrays/old/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/arrays/old/bootconfig.in
kernel.console = tty0, ttyS0
kernel.module_blacklist = "binfmt_misc,i8042"
```

## arrays: new

Input:
```text
kernel.module_blacklist = i8042, binfmt_misc
kernel.console = tty0, ttyS0
```

```console
$ /tmp/symphony-ukisys-redesign-check/target/debug/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/arrays/new/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/arrays/new/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/arrays/new/bootconfig.in
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
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/plain/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/plain/old/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/plain/old/bootconfig.in
kernel.console = tty0, ttyS0
kernel.quiet
```

## plain: new

Input:
```text
kernel.console = tty0, ttyS0
kernel.quiet
```

```console
$ /tmp/symphony-ukisys-redesign-check/target/debug/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/plain/new/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/plain/new/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/plain/new/bootconfig.in
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
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/fips/old/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/fips/old/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/fips/old/bootconfig.in
kernel.fips = 1
kernel.module_blacklist = "binfmt_misc,i8042"
```

## fips: new

Input:
```text
kernel.fips = 1
kernel.module_blacklist = i8042
kernel.module-blacklist = binfmt_misc
```

```console
$ /tmp/symphony-ukisys-redesign-check/target/debug/ukisys merge-bootconfig --input /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/fips/new/bootconfig.in --output /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/fips/new/bootconfig.in
[exit 0]
stdout:

stderr:

$ cat /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/blacklist-cases/fips/new/bootconfig.in
kernel.fips = 1
kernel.module_blacklist = "binfmt_misc,i8042"
```
