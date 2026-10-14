# Commit 06: Update CLI expected blacklist ordering

Revision: `33dbf4bab8ea5810803f05cb7c5d184041be6cf0`. Owning member: Twoliter.

Questions: Q2, Q6. See 02-answers.md.

Only swaps the two expected fixture lines so canonical blacklist appears after other copied text, as accepted. It leaves in-place failure-preservation assertions and module content checks.

## Implications

The CLI fixture now expects the merged blacklist at the end.

Citation: `33dbf4bab8ea5810803f05cb7c5d184041be6cf0:tools/ukisys/tests/cmdline.rs:74`.

Filter: SKIP: restates the accepted ordering change.

## Questions for author

None beyond the explicitly recorded decisions.
