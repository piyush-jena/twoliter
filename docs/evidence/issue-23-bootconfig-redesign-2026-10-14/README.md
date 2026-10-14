# Shared bootconfig validation, 2026-10-14

Final standalone Twoliter PR5 source `43a412a349a5dff0283058d2c2aa50afe4519cd7`
passed both conductor gates using Rust 1.97.1 and `RUSTC_BOOTSTRAP=1`.

| Gate | Result | Request |
| --- | --- | --- |
| Formatting, release workspace tests, strict all-target Clippy | Passed; 572 tests passed, 47 ignored by the suite | `b9eea637-6454-4103-be28-f3082de9aa08` |
| Full `make integ` | 19 passed, 0 failed, 0 ignored | `e23181d7-cedb-4607-9107-94923b6c7c33` |

This includes all 127 pcrsys tests and all three ukisys CLI tests. The workspace
run retained the suite's default ignore settings. The separate integration gate
explicitly ran its ignored integration tests; the 47 workspace ignores are not
claimed as executed by the workspace command.

The final `post-review/gates-43a412a/check/` and `integ/` receipts contain exact
command arguments, exit codes,
source identities and artifact hashes. Their complete logs are included. The top-level `check/` and `integ/` directories
preserve the earlier passing receipts at `33dbf4b` separately. `cli-order-fix.log` records the CLI file-publication
tests and the embedded plain/FIPS assembly fixture. `ukisys-error-console.log`
shows the actual CLI operation and underlying typed bootconfig error.

The final change keeps both Pest grammars unchanged, separates UKI and GRUB
command-line operations, and uses typed Snafu errors in bootconfig and ukisys.
Recursion and Linux's flag/child handling remain. The agreed local text checks
and duplicate-assignment rejection were removed. Both blacklist spellings are
merged into one quoted canonical assignment appended to the drop-in text.

Earlier attempts exposed a missing standalone `ResultExt` import and the old
CLI expectation for blacklist placement. Both were corrected before the passing
receipts above. Earlier logs and diagnoses remain under the stage's
`.symphony/issue24-bootconfig-redesign-2026-10-14/implementation/pr5-checks/`.

The shared changes were also tested with dependent PR6 at
`09380d356886842b8987b602ed2ef7daaefec4f3`: ten fresh x86_64 AMI boots had exact
command-line matches and 256 matching generated PCR comparisons across SHA256
and SHA384, plus 12 separate GRUB PCR9 matches. Detailed live evidence and the
final review are recorded with PR6. Its additional PCR-bank features are not
claimed to exist in standalone PR5.

The single final review found one malformed-blacklist check bypass. The
[resolution](post-review/resolution.md) records its small fix, passing final
gates, actual release regression output, and dependent PR6 replay. All ten
original AMIs remain attributed to `09380d3`; no AMI rebuild is claimed for
the malformed-input rejection fix.
