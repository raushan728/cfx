use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

fn cfx() -> Command {
    Command::cargo_bin("cfx").unwrap()
}

#[test]
fn test_help_and_version() {
    cfx().arg("--help").assert().success();
    cfx().arg("--version").assert().success();
}

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.txt");
    fs::write(&input_path, "secret data").unwrap();

    let output_path = dir.path().join("output.cfx");

    // Encrypt
    cfx()
        .arg("encrypt")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .arg("--no-progress")
        .write_stdin("mypassword\nmypassword\n")
        .assert()
        .success();

    assert!(output_path.exists());
    assert!(!input_path.exists()); // --keep was not used

    // Decrypt
    let decrypted_path = dir.path().join("decrypted.txt");
    cfx()
        .arg("decrypt")
        .arg(&output_path)
        .arg("-o")
        .arg(&decrypted_path)
        .arg("--no-progress")
        .write_stdin("mypassword\n")
        .assert()
        .success();

    assert!(decrypted_path.exists());
    assert_eq!(fs::read_to_string(decrypted_path).unwrap(), "secret data");
}

#[test]
fn test_wrong_password() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.txt");
    fs::write(&input_path, "secret data").unwrap();
    let output_path = dir.path().join("output.cfx");

    cfx()
        .arg("encrypt")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .write_stdin("mypassword\nmypassword\n")
        .assert()
        .success();

    let decrypted_path = dir.path().join("decrypted.txt");
    cfx()
        .arg("decrypt")
        .arg(&output_path)
        .arg("-o")
        .arg(&decrypted_path)
        .write_stdin("wrongpassword\n")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Invalid password")
                .or(predicate::str::contains("Decryption failed")),
        );
}

#[test]
fn test_empty_file() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("empty.txt");
    fs::write(&input_path, "").unwrap();
    let output_path = dir.path().join("empty.cfx");

    cfx()
        .arg("encrypt")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .write_stdin("pass\npass\n")
        .assert()
        .success();

    let decrypted_path = dir.path().join("decrypted.txt");
    cfx()
        .arg("decrypt")
        .arg(&output_path)
        .arg("-o")
        .arg(&decrypted_path)
        .write_stdin("pass\n")
        .assert()
        .success();

    assert_eq!(fs::read(decrypted_path).unwrap().len(), 0);
}

#[test]
fn test_keep_and_force() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("data.txt");
    fs::write(&input_path, "data").unwrap();
    let output_path = dir.path().join("data.cfx");

    // Encrypt with --keep
    cfx()
        .arg("encrypt")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .arg("--keep")
        .write_stdin("p\np\n")
        .assert()
        .success();

    assert!(input_path.exists());

    // Decrypt without force (should fail because input_path already exists and decrypt by default restores to input_path if no -o is provided)
    // Wait, original filename is "data.txt". So decrypting data.cfx without -o will try to write to data.txt.
    cfx()
        .arg("decrypt")
        .arg(&output_path)
        .write_stdin("p\n")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // Decrypt with force
    cfx()
        .arg("decrypt")
        .arg(&output_path)
        .arg("--force")
        .write_stdin("p\n")
        .assert()
        .success();
}

#[test]
fn test_info_and_verify() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("data.txt");
    fs::write(&input_path, "data").unwrap();
    let output_path = dir.path().join("data.cfx");

    cfx()
        .arg("encrypt")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .write_stdin("p\np\n")
        .assert()
        .success();

    cfx().arg("info").arg(&output_path).assert().success();

    cfx()
        .arg("verify")
        .arg(&output_path)
        .write_stdin("p\n")
        .assert()
        .success();

    cfx()
        .arg("verify")
        .arg(&output_path)
        .write_stdin("wrong\n")
        .assert()
        .failure();
}

#[test]
fn test_recover() {
    let dir = tempdir().unwrap();
    let tmp_path = dir.path().join("interrupted.cfx.tmp");
    fs::write(&tmp_path, "junk").unwrap();

    cfx()
        .arg("recover")
        .arg(&tmp_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleaning up"));

    assert!(!tmp_path.exists());
}
