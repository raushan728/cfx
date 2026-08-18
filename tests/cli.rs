use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_top_level_help() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Secure file encryption and decryption utility",
        ))
        .stdout(predicate::str::contains("Commands:"));
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn test_command_specific_help() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("encrypt")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Encrypt a file"))
        .stdout(predicate::str::contains("-o, --output <PATH>"));
}

#[test]
fn test_missing_required_input() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("encrypt")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "the following required arguments were not provided:",
        ));
}

#[test]
fn test_encrypt_with_all_options() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("encrypt")
        .arg("input.txt")
        .arg("--output")
        .arg("output.cfx")
        .arg("--force")
        .arg("--keep")
        .arg("--no-progress")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_decrypt_with_all_options() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("decrypt")
        .arg("input.cfx")
        .arg("-o")
        .arg("output.txt")
        .arg("-f")
        .arg("--keep")
        .arg("--no-progress")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_info_required_argument() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("info")
        .arg("file.cfx")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_verify_with_options() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("verify")
        .arg("file.cfx")
        .arg("--no-progress")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_recover_with_all_options() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("recover")
        .arg("corrupted.cfx")
        .arg("--output")
        .arg("recovered.txt")
        .arg("--force")
        .arg("--keep")
        .arg("--no-progress")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_invalid_option() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("encrypt")
        .arg("input.txt")
        .arg("--unknown-option")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}

#[test]
fn test_unknown_command_fails() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("unknown-command").assert().failure();
}
