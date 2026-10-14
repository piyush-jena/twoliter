# Q4: Retained byte and formatting boundaries

Traced binary trailer/magic/size/checksum/padding and UTF-8 checks in parse; namespace output checks in parse_text; format_params quote rejection; UKI <2048-byte limit; LoadOptions ASCII/NUL check; and .cmdline termination/padding/UTF-8 checks. pcrsys still extracts PE sections and rejects prediction-specific EFI/initrd overrides. Base argument validation removal and broader text acceptance are explicit decisions, not findings. Successful GRUB formatting, whitespace and newline behavior were independently exercised on the six live fixtures.

Evidence: 09380d356886842b8987b602ed2ef7daaefec4f3:tools/bootconfig/src/lib.rs:42; 09380d356886842b8987b602ed2ef7daaefec4f3:tools/bootconfig/src/lib.rs:177; 09380d356886842b8987b602ed2ef7daaefec4f3:tools/bootconfig/src/parsers/bootconfig.rs:139; 09380d356886842b8987b602ed2ef7daaefec4f3:tools/bootconfig/src/uki_cmdline.rs:10; 09380d356886842b8987b602ed2ef7daaefec4f3:tools/pcrsys/src/pe.rs:232.
