# Commit 01: Explain and test module blacklist parameter alias merging

Revision: `89f5bc36aed69803a2428a2494fb30a93fcd83fa`. Owning member: Twoliter.

Questions: Q1, Q2, Q3. See 02-answers.md.

Compared parent 515d9ae. Production change is explanatory comments; the fixture uses disjoint modules under both spellings. Historical evidence is explicitly dated October 13 and does not claim to validate this redesign.

## Implications

Both spellings are still recognized and emitted under the canonical underscore spelling.

Citation: `89f5bc36aed69803a2428a2494fb30a93fcd83fa:tools/bootconfig/src/dropins.rs:8`.

Filter: SKIP: restates the documented behavior and existing alias matcher.

## Questions for author

None beyond the explicitly recorded decisions.
