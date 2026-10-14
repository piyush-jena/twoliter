# Q6: Final gates and failed attempts

Read final build results, source provenance and logs, and independently verified input, log and artifact SHA256s. PR5 check: 572 passed, 47 ignored; PR6 check: 593 passed, 47 ignored; both integration runs: 19 passed. Check logs include fmt and strict Clippy. Tool build 5f9981c7 is successful at PR6. harness-integ-network.log ends with all gates passed and exit=0. Earlier missing-import and CLI-order failures remain historical evidence, not counted as successes. No gates were rerun.

Evidence: gates-checked.json; ../implementation/harness-integ-network.log; ../implementation/standalone-gates.json:2; ../implementation/integrated-gates.json:2.
