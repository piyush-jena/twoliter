# Issue 24 final-review responses, 2026-10-13

The single final adversarial review of Twoliter
`59bf81b96bcde9cc83ebe9039b8bf3b344ea5367` completed. Its independent replay
confirmed 640/640 TPM comparisons, 320 per bank, and the firmware-derived
PCR0 SHA-384 constant. No further adversarial review is requested.

## Findings

- **F1, closed in source; check gate pending.** The independent Python oracle
  now constructs the existing `build_test_shim` fixture, hashes its explicit
  Authenticode byte ranges including the COFF overlay, and predicts GRUB PCR4
  by extending action, separator, shim, GRUB and kernel. The Rust fixture
  supplies that PE for all three boot images and checks PCR4 against the
  fixed SHA-256 and SHA-384 values. The generated SHA-256 result matches the
  preexisting `pcr4.rs` AWS expectation, `b60bad6f...839a72`.
  Files: `tools/pcrsys/src/test_banks.rs`,
  `tools/pcrsys/tests/data/generate-bank-vectors.py` and `bank-vectors.json`.
- **F2, addressed by disclosure.** `tools/pcrsys/README.md` now explicitly
  states that VMware/metal SHA-384 branches lack platform-specific
  expected-value tests and runtime qualification. Preserve this limitation
  in the final PR body; no VMware or metal runtime result is claimed.
- **F3, intentional behavior documented.** The supported UKI route remains
  AWS-only. README now names the exact error and states that Secure Boot UKI
  builds and repacks for VMware/metal fail at prediction. No runtime behavior
  changed.
- **F4, corrected evidence labeling.** The collected `pcr0-evidence.json`
  nests October 7 event-log revisions inside `event_log` and October 10
  runtime-capture revisions inside `receipt_source`. Original source
  provenance was read again to confirm both identities. The original proof
  archive remains immutable; the final refreshed archive must include this
  corrected JSON and supersede the old proof hash.
- **F5, final publication pending checks.** The conductor PR request must
  identify the final commit, 640 comparisons, the refreshed archive/hash,
  inherited October 10 captures, and the platform/workload limitations.
  Preserve the dependency on bootconfig PR5/issue23 and keep both issues open.
- **F6, retained with rationale.** The repository instruction prohibits the
  removed punctuation. The header's verbatim-copy requirement refers to the
  dictionary bytes, not the Rust explanatory comment. Everything from
  `use base64` onward is byte-identical to the PCR split base
  `5b61b5138eff42ea786b6345a164d56729508676`; its SHA-256 is
  `1336aba3f81f734f25e3f89e3f990f7e157ad7ea4e31cfaa562a2b610867daa8`.
  No dictionary code or data changed.

## Affected validation

The follow-up changes only a private test module, its Python/JSON fixtures,
README and evidence documentation. Production prediction code, dependencies,
image construction and command-line generation are unchanged from the
release-tested `59bf81b` revision. `tools/pcrsys/src/main.rs:16` at that
revision compiles `test_banks` only under `cfg(test)`; the vectors are consumed
only by that module.

Run the conductor check and full integration gates at the follow-up commit.
The new PCR4 expectation must execute as part of the check gate. No release
binary or image rebuild is needed for these test/documentation changes.
The existing pcrsys binary and 640 comparisons retain their exact `59bf81b`
identity; do not relabel them as execution of the follow-up commit. No live
validation is affected, and none is rerun for this change.

Before publishing the PR update, record the completed gate receipts, the
complete changed-file list establishing unchanged production source, and a
refreshed proof archive. Return `in-review` with `pr_requests` after the gates
pass; the final review is already complete.
