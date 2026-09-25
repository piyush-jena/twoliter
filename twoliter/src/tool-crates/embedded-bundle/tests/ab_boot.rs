use std::path::Path;
use std::process::Command;

#[test]
fn authenticated_gpt_image_constraints() {
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("embedded/tests/test-ab-boot.sh");
    let output = Command::new("bash").arg(script).output().unwrap();
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
