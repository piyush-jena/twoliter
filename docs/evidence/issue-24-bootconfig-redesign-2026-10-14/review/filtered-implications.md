# Filtered implications

## Correctness

| Commit | SHA | Implication | Citation |
| --- | --- | --- | --- |
| 02 | 0487150d | A block-form kernel.module_blacklist followed by a supported assignment passes the retained form check because last-value replacement hides the block; the successful output still contains two blacklist assignments. | tools/bootconfig/src/lib.rs:159 |

## Assumptions

No additional surfaced implications. Fresh-boot limitations are recorded in REVIEW.md.

## Type system opportunities

No additional surfaced implications within the accepted redesign.

## Intent (needs author)

None.

## Skipped

| Implication | Skip reason |
| --- | --- |
| Both spellings are still recognized and emitted under the canonical underscore spelling. | SKIP: restates the documented behavior and existing alias matcher. Citation: 89f5bc36:tools/bootconfig/src/dropins.rs:8. |
| A block-form kernel.module_blacklist followed by a supported assignment passes the retained form check because last-value replacement hides the block; the successful output still contains two blacklist assignments. | SKIP as duplicate of commit 02/F1; final PR6 is affected. Citation: 1c7caf56:tools/bootconfig/src/lib.rs:159. |
| This merge adds historical issue-23 evidence without changing the integrated source tree. | SKIP: restates first-parent diff. Citation: 1630ab81:docs/evidence/issue-23-review-2026-10-13.md:1. |
| Standalone pcrsys gains the trait import needed by its typed error context call. | SKIP: restates the one-line fix and resolved historical failure. Citation: 01c1e3f0:tools/pcrsys/src/pcrs/pcr9.rs:13. |
| The CLI fixture now expects the merged blacklist at the end. | SKIP: restates the accepted ordering change. Citation: 33dbf4ba:tools/ukisys/tests/cmdline.rs:74. |
| The final merge carries the already-reviewed CLI fixture ordering into PR6. | SKIP: restates merge diff; duplicates commit 06. Citation: 09380d35:tools/ukisys/tests/cmdline.rs:74. |
