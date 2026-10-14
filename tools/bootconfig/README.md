# bootconfig

Bootconfig parsing and boot command-line construction shared by `ukisys` and `pcrsys`.

## Usage

Use `parse` for standalone binary bootconfig produced by the SDK tool, or `parse_text` for text.

`merge_blacklisted_modules` combines package blacklist contributions before binary encoding.
It accepts both parameter spellings and appends one quoted `kernel.module_blacklist` assignment.
`uki_cmdline` combines bootconfig and base arguments, separating kernel and init arguments with `--`.
`parse_uki_cmdline` decodes an already-extracted UKI command-line section; PE inspection stays with the caller.
`uki_load_options` produces the UTF-16LE measurement buffer.
`predict_grub_cmdline` handles GRUB variables, boot-partition UUID substitution, and quote repair.

## Input

The parser supports flags, quoted values, arrays, and nested keys in Linux bootconfig order.
Later assignments replace earlier values at the same key. Flags with child settings remain namespaces.
The unchanged Pest grammars define accepted syntax; the text parser adds no size, character, or nesting pre-scan.
Image-builder base arguments are trusted. Composed UKI command lines must fit within 2048 bytes including NUL.
Binary size, checksum, padding and UTF-8 errors are still checked.
The public `error::Error` enum reports the failed operation and preserves parser and UTF-8 error sources.
See the API documentation for syntax restrictions.
