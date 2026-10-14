# Commit 03: Integrated bootconfig path refactor

Revision: `1c7caf566e0dd68926ef6da5017d505f586b6073`. Owning member: Twoliter.

Questions: Q2, Q3, Q4, Q5, Q8. See 02-answers.md.

Compared parent 661f4abb. Includes shared refactor plus pcrsys PE section decoding delegation, error-source wrappers and tests. Prediction-specific EFI/initrd restrictions remain in pcrsys. The same merger issue is inherited by final PR6.

## Implications

A block-form kernel.module_blacklist followed by a supported assignment passes the retained form check because last-value replacement hides the block; the successful output still contains two blacklist assignments.

Citation: `1c7caf566e0dd68926ef6da5017d505f586b6073:tools/bootconfig/src/lib.rs:159`.

Filter: SKIP as duplicate of commit 02/F1; final PR6 is affected.

## Questions for author

None beyond the explicitly recorded decisions.
