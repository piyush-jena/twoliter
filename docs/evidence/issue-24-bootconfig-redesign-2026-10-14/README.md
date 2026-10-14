# Bootconfig redesign validation, 2026-10-14

Twoliter `09380d356886842b8987b602ed2ef7daaefec4f3` passed all ten existing
x86_64 variants. Start with [the report](report.md). Each `rows/<variant>/console.md`
contains the exact commands, exit codes, stdout and stderr. Its comparison JSON
records both banks, the selected prediction states, health checks and exclusions;
`expected-cmdline.txt` preserves the expected bytes including the final newline.

There are 256 matching generated bank/register comparisons and 12 separate GRUB
PCR9 matches. PCR9 remains absent from the generated A/B predictions. Build,
local-disk and registered-AMI predictions agree; every live command line matches
exactly. FIPS and NVIDIA checks apply to their respective variants.

This directory contains the report, console transcripts and comparisons. Full
SSM/API receipts, build logs and validation scripts remain under the stage's
`.symphony/issue24-bootconfig-redesign-2026-10-14/validation/`. Their locations and
hashes are recorded in `full-evidence-pointers.json` and `helper-provenance.json`.
`manifest.json` records this selection's hashes and the complete live-matrix
bundle's manifest hash. Review and post-review artifacts are identified separately.
The console commands preserve their actual validation-host paths and require its
retained tools, artifacts and AWS credentials to replay.

For the live-tested revision, both source-check and integration gates passed with Rust 1.97.1. The integrated
workspace run reported 593 tests passed and 47 ignored by the suite; the separate
integration gate passed all 19 tests. The 47 default-ignored tests are not claimed
as executed by the workspace run. The source check receipt is included under
`prerequisites/check/`. The initial missing standalone trait import and old CLI
ordering expectation were fixed before these final checks and image builds.

Limits: one fresh ready boot per variant, x86_64 only. No workload, reboot-cycle,
A/B-slot-switch or update-cycle validation is claimed by this live matrix. The
checkout has no ECS 4 FIPS variants. Unpredicted PCRs are recorded observations,
not passing predictions. New AMIs and instances are retained; previous resources
were not modified. No raw images, private keys, binaries or runtime archives are
published here.

The single [final review](review/REVIEW.md) identified one malformed-input check
bypass. Its [resolution](post-review/resolution.md) records the small fix and
separate validation. The live matrix source remains `09380d3`; later validation
is attributed to its own revision.

Final source `830ee975` passed formatting, 593 workspace tests (47 suite-default
ignores), strict Clippy and all 19 integration tests. Its release tools
[replayed all ten retained images](post-review/replay-830ee/report.md), reproducing
all prediction JSON and command-line bytes, and the same 256 generated and
12 separate GRUB PCR9 comparisons. The release CLI passes the malformed-input
regression and five byte-identical supported controls. F1 is addressed.
