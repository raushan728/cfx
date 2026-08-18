use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_succeeds() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn test_version_succeeds() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn test_encrypt_placeholder() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("encrypt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_decrypt_placeholder() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("decrypt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_info_placeholder() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("info")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_verify_placeholder() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("verify")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_recover_placeholder() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("recover")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented yet"));
}

#[test]
fn test_unknown_command_fails() {
    let mut cmd = Command::cargo_bin("cfx").unwrap();
    cmd.arg("unknown-command").assert().failure();
}
