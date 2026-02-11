use super::run_command;
use std::path::PathBuf;
use tempfile::TempDir;

pub const ADVISORY_CHECKER_PATH: &str = env!("CARGO_BIN_FILE_ADVISORY_CHECKER");

const SPEC_TEMPLATE: &str = r#"Name: {name}
Version: {version}
Release: 1
Summary: Test package
License: MIT

%description
Test package
"#;

fn create_spec_file(dir: &TempDir, name: &str, version: &str) -> PathBuf {
    let content = SPEC_TEMPLATE
        .replace("{name}", name)
        .replace("{version}", version);
    let path = dir.path().join("test.spec");
    std::fs::write(&path, content).unwrap();
    path
}

fn create_advisory_file(dir: &TempDir, filename: &str, content: &str) -> PathBuf {
    let path = dir.path().join(filename);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(&path, content).unwrap();
    path
}

#[test]
#[ignore]
fn test_missing_advisories_dir_succeeds() {
    // Users of kits may want to do away with the advisories directory.
    let temp_dir = TempDir::new().unwrap();
    let spec_file = create_spec_file(&temp_dir, "testpkg", "1.0.0");

    // We provide an arbitrary advisories directory that isn't present.
    let advisories_dir_name = "nonexistent";
    let nonexistent_dir = temp_dir.path().join(advisories_dir_name);

    // We run the advisory-checker here
    let output = run_command(
        ADVISORY_CHECKER_PATH,
        [
            "--spec-file",
            spec_file.to_str().unwrap(),
            "--advisories-dir",
            nonexistent_dir.to_str().unwrap(),
        ],
        [],
    );

    // The step passes becauses we ignore advisory checks in this case.
    assert!(output.status.success());

    // We also test the expected output to make sure that the code path is as expected.
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("skipping"));
}

#[test]
#[ignore]
fn test_empty_advisories_dir_succeeds() {
    // Given a scenario when the advisory directory is empty
    let temp_dir = TempDir::new().unwrap();
    let spec_file = create_spec_file(&temp_dir, "testpkg", "1.0.0");

    let advisories_dir = temp_dir.path().join("advisories");
    std::fs::create_dir(&advisories_dir).unwrap();

    // We run the advisory-checker here
    let output = run_command(
        ADVISORY_CHECKER_PATH,
        [
            "--spec-file",
            spec_file.to_str().unwrap(),
            "--advisories-dir",
            advisories_dir.to_str().unwrap(),
        ],
        [],
    );

    // Then the test will pass because the logic will search for toml
    // files and won't find any and exit without errors.
    assert!(output.status.success());
}

#[test]
#[ignore]
fn test_ignores_non_toml_files() {
    // Given a scenario when the advisory directory has non-toml files
    let temp_dir = TempDir::new().unwrap();
    let spec_file = create_spec_file(&temp_dir, "testpkg", "1.0.0");
    let advisories_dir = temp_dir.path().join("advisories");
    std::fs::create_dir(&advisories_dir).unwrap();
    std::fs::write(advisories_dir.join(".gitkeep"), "").unwrap();

    // We run the advisory-checker here
    let output = run_command(
        ADVISORY_CHECKER_PATH,
        [
            "--spec-file",
            spec_file.to_str().unwrap(),
            "--advisories-dir",
            advisories_dir.to_str().unwrap(),
        ],
        [],
    );

    // Then the test will pass because the logic will search for toml
    // files and won't find any and exit without errors.
    assert!(output.status.success());
}

#[test]
#[ignore]
fn test_advisory_violation_fails() {
    // Given a scenario when we have a software package that is at a lower version
    // than a published advisory.
    let temp_dir = TempDir::new().unwrap();
    let advisories_dir = temp_dir.path().join("advisories");
    std::fs::create_dir(&advisories_dir).unwrap();

    let spec_file = create_spec_file(&temp_dir, "testpkg", "1.0.0");

    let advisory = r#"[advisory]
id = "BRSA-test123"
title = "Test Advisory"
cve = "CVE-2024-12345"
severity = "high"
description = "Test vulnerability"

[[advisory.products]]
package-name = "testpkg"
patched-version = "2.0.0"
patched-epoch = "0"
"#;
    create_advisory_file(&temp_dir, "advisories/BRSA-test.toml", advisory);

    // We run the advisory-checker here
    let output = run_command(
        ADVISORY_CHECKER_PATH,
        [
            "--spec-file",
            spec_file.to_str().unwrap(),
            "--advisories-dir",
            advisories_dir.to_str().unwrap(),
        ],
        [],
    );

    // Then the code runs successfully but it finds advisory violations
    // because the package version is lower than the advisory
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Advisory violations found"));
}

#[test]
#[ignore]
fn test_advisory_satisfied_succeeds() {
    // Given a scenario when we have a software package that is at a higher
    // version than its published advisory.
    let temp_dir = TempDir::new().unwrap();
    let advisories_dir = temp_dir.path().join("advisories");
    std::fs::create_dir(&advisories_dir).unwrap();

    let spec_file = create_spec_file(&temp_dir, "testpkg", "3.0.0");

    let advisory = r#"[advisory]
id = "BRSA-test456"
title = "Test Advisory"
cve = "CVE-2024-67890"
severity = "moderate"
description = "Test vulnerability"

[[advisory.products]]
package-name = "testpkg"
patched-version = "2.0.0"
patched-epoch = "0"
"#;
    create_advisory_file(&temp_dir, "advisories/BRSA-test.toml", advisory);

    // We run the advisory-checker here
    let output = run_command(
        ADVISORY_CHECKER_PATH,
        [
            "--spec-file",
            spec_file.to_str().unwrap(),
            "--advisories-dir",
            advisories_dir.to_str().unwrap(),
        ],
        [],
    );

    // Then the code runs successfully and doesn't find advisory violations.
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("Advisory violations found"));
}

#[test]
#[ignore]
fn test_advisory_for_removed_package_ignored() {
    // Given a scenario when we have a kit and advisories for removed packages
    let temp_dir = TempDir::new().unwrap();
    let advisories_dir = temp_dir.path().join("advisories");
    std::fs::create_dir(&advisories_dir).unwrap();

    let spec_file = create_spec_file(&temp_dir, "otherpkg", "1.0.0");

    let advisory = r#"[advisory]
id = "BRSA-test789"
title = "Test Advisory"
cve = "CVE-2024-11111"
severity = "critical"
description = "Test vulnerability"

[[advisory.products]]
package-name = "testpkg"
patched-version = "2.0.0"
patched-epoch = "0"
"#;
    create_advisory_file(&temp_dir, "advisories/BRSA-test.toml", advisory);

    // We run the advisory-checker here
    let output = run_command(
        ADVISORY_CHECKER_PATH,
        [
            "--spec-file",
            spec_file.to_str().unwrap(),
            "--advisories-dir",
            advisories_dir.to_str().unwrap(),
        ],
        [],
    );

    // Then the code runs successfully and doesn't test on the advisories
    // not meant for this package
    assert!(output.status.success());
}

#[test]
#[ignore]
fn test_parse_advisory_error_invalid_toml() {
    // Given a spec file and an advisory file with invalid TOML
    let temp_dir = TempDir::new().unwrap();
    let advisories_dir = temp_dir.path().join("advisories");
    std::fs::create_dir(&advisories_dir).unwrap();

    let spec_file = create_spec_file(&temp_dir, "testpkg", "1.0.0");
    create_advisory_file(
        &temp_dir,
        "advisories/BRSA-invalid.toml",
        "not valid toml {{{",
    );

    // We run the advisory-checker here
    let output = run_command(
        ADVISORY_CHECKER_PATH,
        [
            "--spec-file",
            spec_file.to_str().unwrap(),
            "--advisories-dir",
            advisories_dir.to_str().unwrap(),
        ],
        [],
    );

    // The command fails because of a failure in advisory parsing and
    // we want the user to fix/remove the advisory.
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to parse advisory"));
}
