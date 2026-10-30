# bootconfig

A library for bootconfig parsing and kernel command-line construction.

## Usage

Every operation a consumer performs is a function in `lib.rs`; the parsing, composition, and
prediction behind them are internal to the crate.

`merge_parameters` combines the `kernel.module_blacklist` and `kernel.initcall_blacklist` values
packages contribute into one comma-separated value each, before binary encoding. Every other
assignment is kept as written.
`compose_uki_cmdline` builds the command line to embed in a UKI from binary bootconfig and the
base arguments of the image, joined by single spaces. It rejects a line that collapsing whitespace
would change, one with an unpaired quote, and one too long for the kernel.
`predict_grub_cmdline` predicts the command line GRUB starts a kernel with, as UTF-16LE EFI
LoadOptions without a terminating NUL, and the command line the kernel then reports, for measuring PCR 9.
`decode_uki_cmdline` decodes an already-extracted UKI command-line section; PE inspection, and
the policy on which command lines may be measured, stay with the caller.
`uki_kernel_parameters` names the kernel parameters of a UKI command line, split as the kernel
splits them, for that policy to inspect.
`encode_uki_load_options` produces the UTF-16LE measurement buffer that systemd-stub measures from
the `.cmdline` section of the UKI.
