use std::process::Command;

#[test]
fn update_query_emits_only_a_boolean_even_with_experimental_features() {
    let dir = tempfile::tempdir().unwrap();
    let manifest = dir.path().join("Cargo.toml");
    for enabled in [true, false] {
        std::fs::write(
            &manifest,
            format!(
                r#"
[package]
name = "aws-mantle-1"
version = "0.1.0"
[package.metadata.build-variant]
image-format = "uki"
[package.metadata.build-variant.image-features]
in-place-updates = {enabled}
encrypted-storage = true
ephemeral-encryption-keys = true
uefi-secure-boot = true
"#
            ),
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_buildsys"))
            .args(["variant-in-place-updates", "--variant-manifest"])
            .arg(&manifest)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, format!("{enabled}\n").as_bytes());
    }
}
