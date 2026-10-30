//! Integration test that runs every bash test suite in `embedded/tests`.
//!
//! The assertions live in the `embedded/tests/test_*.sh` scripts, each of which
//! exercises one embedded script or helper library. This Rust wrapper finds and
//! runs them all, so that a new suite runs under `cargo test` as soon as it is
//! added.

use std::path::PathBuf;
use std::process::Command;

#[test]
fn bash_suites() {
    // The `embedded` directory is a symlink to `twoliter/embedded` at the
    // crate root, so `CARGO_MANIFEST_DIR` reaches the scripts either way.
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("embedded/tests");
    let mut suites: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", dir.display()))
        .map(|entry| entry.expect("failed to read a directory entry").path())
        .filter(|path| {
            let name = path.file_name().and_then(|name| name.to_str());
            name.is_some_and(|name| name.starts_with("test_") && name.ends_with(".sh"))
        })
        .collect();
    suites.sort();
    assert!(!suites.is_empty(), "no test suites in {}", dir.display());

    let mut failed = Vec::new();
    for suite in &suites {
        let name = suite
            .file_name()
            .expect("a suite has a file name")
            .to_string_lossy();
        let output = Command::new("bash")
            .arg(suite)
            .output()
            .expect("failed to invoke bash");

        // Always print so failures show what the bash runner saw.
        println!(
            "--- {name} stdout ---\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        if !output.stderr.is_empty() {
            println!(
                "--- {name} stderr ---\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        if !output.status.success() {
            failed.push(format!("{name} ({})", output.status));
        }
    }
    assert!(failed.is_empty(), "failed suites: {}", failed.join(", "));
}
