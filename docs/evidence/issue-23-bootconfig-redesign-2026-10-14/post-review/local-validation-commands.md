# Local post-review validation

These checks use isolated manifests pointing to the actual stage source. Their
locked dependencies are cached locally; the conductor gates separately test the
full repository workspace. The Rust toolchain is 1.97.1.

```sh
PATH=/home/fedora/Repositories/symphony/.symphony/tools/rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/bin:$PATH cargo test --offline --manifest-path /tmp/symphony-ukisys-redesign-check/Cargo.toml --target-dir /tmp/symphony-ukisys-redesign-check/target -- --skip rpm2img_wires_plain_and_fips_bootconfig_before_uki_assembly
```

Exit 0. Exact captured output: `ukisys-local.log` (8 unit tests, 2 CLI tests,
1 workspace-path-dependent test filtered in this isolated run).

From the PR6 Twoliter checkout:

```sh
bash twoliter/embedded/tests/test_uki_cmdline.sh /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/twoliter/twoliter/embedded /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/twoliter/tools/ukisys/tests/fixtures /tmp/symphony-ukisys-redesign-check/target/debug/ukisys /tmp/symphony-post-review-shell-fixture
```

Exit 0. Exact captured output: `embedded-cli-fixture.log`. This executes the
plain/FIPS shell fixture directly despite the isolated manifest location.

```console
$ python3 stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/verify-blacklist-fix.py
PASS: mixed block-form input rejected without changing its file; five supported cases byte-identical.
```

Exit 0. Each underlying CLI invocation and its exact output is recorded in
`blacklist-console.md`; machine-readable receipts include the binary hashes.
Replay the script at its original validation-host path shown above; the copied
script and logs preserve that execution rather than providing portable binaries.
