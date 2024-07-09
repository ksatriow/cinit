use std::process::Command;

#[test]
fn test_cinit_version() {
    let output = Command::new("./target/release/cinit")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let version = String::from_utf8(output.stdout).expect("Failed to read stdout");

    assert!(version.contains("1.0"), "Version string did not contain '1.0': {}", version);
}
