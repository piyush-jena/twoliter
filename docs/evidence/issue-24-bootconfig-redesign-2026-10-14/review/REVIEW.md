# Final adversarial review, 2026-10-14

One actionable finding in the local candidates. No additional actionable finding in the ten-row validation evidence.

## Correctness: F1 - Check blacklist form before coalescing assignments

Location: Twoliter PR5 `33dbf4bab8ea5810803f05cb7c5d184041be6cf0:tools/bootconfig/src/lib.rs:159` and PR6 `09380d356886842b8987b602ed2ef7daaefec4f3:tools/bootconfig/src/lib.rs:159`. The relevant final condition is lines 165-168; replacement occurs at `tools/bootconfig/src/parsers/bootconfig.rs:33` in both candidates.

`merge_blacklisted_modules()` copies unrecognized block-form input, appends the canonical assignment, then validates the parsed result. Because `Node::insert()` now replaces earlier values, parsing hides an earlier block-form `module_blacklist` behind the appended value. The check sees one parameter and returns success even though its output retains both assignments.

Reproduced using the final extracted `ukisys`:

```text
kernel { module_blacklist = evil }
kernel.module_blacklist = i8042
```

Exit 0, output:

```text
kernel { module_blacklist = evil }
kernel.module_blacklist = "i8042"
```

The block alone exits 1 with `Module blacklist must be a single-line assignment`. The old baseline rejected this mixed input through its duplicate-value guard (`515d9ae99a8800eb0373d7f6e33a41183435c506:tools/bootconfig/src/lib.rs:68`). General duplicate replacement is an accepted redesign decision; preserving the separate blacklist form check requires validating unmerged blacklist occurrences before canonical append/coalescing. Add a mixed block/assignment regression alongside the block-only case at `09380d356886842b8987b602ed2ef7daaefec4f3:tools/bootconfig/src/test_lib.rs:119`.

Evidence: [mixed probe](mixed.probe.json), [block-only probe](block-only.probe.json), [normal alias control](canonical.probe.json). The successful mixed output is the observed defect; no downstream SDK failure or runtime blacklist loss is claimed by this probe. The ten generated-image fixtures do not contain this malformed combination.

## Independently checked

- Reviewed all seven unique commits and both merges in the assigned increments. Verified both Pest grammars unchanged and shared bootconfig/ukisys trees identical between candidates. Retained binary, quote, namespace, UKI length, LoadOptions and pcrsys-specific boundaries were traced; accepted removals were not treated as regressions.
- Verified final source-check, integration and tool-build receipts, including input/log/artifact hashes. PR5: 572 passed, 47 ignored; PR6: 593 passed, 47 ignored; both integrations: 19 passed. Read the successful root `make integ` log. Historical failures were not counted as passing validation.
- Rehashed 386 image artifacts totaling 14,225,226,110 bytes and compared 62 implementation/manifests files in each of ten build input archives to PR6. Checked tool hashes, tool extraction receipts, build source identities, registered AMI/instance mapping, account `533267423195`, runtime region `us-west-2`, snapshot mappings and registered EFI data. Build, local-disk and direct registered-AMI prediction JSON agree in all ten rows.
- Independently parsed all 48 PCR readings per instance and checked 146 successful SSM readbacks. Recompared **256 generated values, 128 per bank**, with common prediction states across banks. PCR11 used index 4 from observed readiness, stable boot IDs and phase-unit receipts, not any matching phase.
- Compared complete `/proc/cmdline` SSM bytes, including final newline: GRUB 760/797/762/799/760/762 bytes; UKI 820/857/822/859 bytes in summary row order. Reran the six pure local GRUB formatter commands and decoded the four retained signed UKI `.cmdline` sections independently. Recomputed **12 separate GRUB PCR9 diagnostics** from build-derived command-line bytes; these are excluded from generated A/B JSON/counts. Unpredicted registers remain separate.
- Checked captured readiness, running system, no failed units, release, architecture and Secure Boot; FIPS flags on FIPS variants; driver and device enumeration on NVIDIA variants. These support the report's fresh-boot health observations.

Machine-readable checks and reproducible read-only audit: [independent-evidence-checks.json](independent-evidence-checks.json), [verify-evidence.py](verify-evidence.py), [gates-checked.json](gates-checked.json). Question/answer trail: [02-answers.md](02-answers.md). Per-commit coverage: `03-commit-01.md` through `03-commit-07.md`. Implication filter: [filtered-implications.md](filtered-implications.md).

## Scope and limits

PR5 reviewed `515d9ae99a8800eb0373d7f6e33a41183435c506..33dbf4bab8ea5810803f05cb7c5d184041be6cf0`; PR6 reviewed `661f4abb11ad312801bef5684c199154769d4d09..09380d356886842b8987b602ed2ef7daaefec4f3`, including interactions with existing PCR prediction code. Publication metadata records the old remote heads and bases; the new local PR5 is an ancestor of the new local PR6. This review does not claim the candidates are published.

The matrix is ten existing x86_64 variants at fresh-boot current states, with six GRUB and four UKI paths. It does not live-test all generated alternatives, reboot/update transitions, workloads, GPU computation or other architectures, and does not qualify upstream PR1076 packaging/service ownership changes. ECS4 FIPS variants are absent from the inventory. Saved AWS/SSM receipts were audited locally; no new cloud query, build, reboot, expensive gate or upload was run. Source and shared files were not edited. All review writes are confined to this directory.

## Proposed evidence publication selection

Read both publication-ready README files and the PR6 report after the bundle became available. Verified all 1134 files against complete-bundle manifest `e895620eaeb5e42211d093fee5e27a8959f2bdd02ce01694cff7f5edbf793b5c`, all 42 selected-file hashes, helper provenance hashes, and byte identity of all ten selected consoles, comparison JSONs and expected command-line files with canonical runs. The selected summary equals the audited canonical summary. No additional actionable finding in these reports. These are prepared files, not a claim of publication. See [publication-selection-checks.json](publication-selection-checks.json).
