# Review finding F1: resolution

The single final adversarial review found that a malformed block-form blacklist
could be hidden by the appended canonical assignment during last-value replacement.
The original finding and its evidence are preserved under `../review/`.

Twoliter PR5 `43a412a349a5dff0283058d2c2aa50afe4519cd7` moves the retained
blacklist-form check before the canonical append in `tools/bootconfig/src/lib.rs`.
It adds a mixed block/assignment regression to `tools/bootconfig/src/test_lib.rs`.
PR6 incorporates that fix at `830ee97501b05adbf6dc76d13383643e440a6002`.
The normal parser, formatter, PCR calculations and both grammars are unchanged
from the live-tested revision. General duplicate-value replacement remains.

`blacklist-console.md` contains exact old/new CLI commands, exit codes and output.
The release binary used for the live matrix accepts the malformed combination;
the fixed local binary rejects it and leaves the input file unchanged. Five
supported input cases (canonical, aliases, arrays, plain and FIPS) have identical
output bytes. `blacklist-comparison.json` records the inputs and binary hashes.
The reproduction script retains its actual host binary paths.

Local Rust 1.97.1 checks passed: 45 bootconfig tests, strict bootconfig Clippy,
8 ukisys tests and 2 CLI file tests. The embedded plain/FIPS shell fixture also
passed when run directly; the isolated manifest cannot resolve its workspace
fixture path, so that one integration test was skipped in the isolated run only.
The conductor workspace gates run the full CLI suite.

The ten live AMIs were built with `09380d356886842b8987b602ed2ef7daaefec4f3`.
They were not rebuilt for this malformed-input rejection fix. Final-source gates passed. The final release tools reproduced all ten
prediction JSON files byte-for-byte, all command-line bytes, 256 generated
comparisons (128 per bank), and 12 separate GRUB PCR9 diagnostics. The actual
release CLI also rejected the regression case while preserving its input and
produced identical output for all five supported controls. No additional adversarial review round was run.

Final-source results and exact replay commands: [replay report](replay-830ee/report.md).
The final PR6 check passed 593 tests with 47 suite-default ignores, formatting
and strict Clippy; its integration gate passed all 19 tests. PR5 final-source
checks passed 572 tests with 47 suite-default ignores and all 19 integration
tests, recorded in its corresponding evidence directory.
