use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
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
fn test_recover_exact_file() {
    let dir = tempdir().unwrap();

    // 1. Unrelated tmp file
    let junk_path = dir.path().join("junk.tmp");
    fs::write(&junk_path, "junk").unwrap();

    cfx()
        .arg("recover")
        .arg(&junk_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("could not be verified"));

    assert!(junk_path.exists());

    // 2. Genuine CFX temporary file (with CFX~ marker)
    let tmp_path = dir.path().join("interrupted.cfx.tmp");
    let mut f = fs::File::create(&tmp_path).unwrap();
    // Write CFX~ header
    f.write_all(b"CFX~").unwrap();
    f.write_all(&[1, 1, 1]).unwrap(); // version, kdf, cipher
    f.write_all(&[0u8; 32]).unwrap(); // salt
    f.write_all(&[0u8; 8]).unwrap(); // nonce
    f.sync_all().unwrap();

    // Test rejection without force (simulated by stdin being empty)
    cfx()
        .arg("recover")
        .arg(&tmp_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Skipped deletion."));

    assert!(tmp_path.exists());

    // Test successful deletion with --force
    cfx()
        .arg("recover")
        .arg(&tmp_path)
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleanup successful."));

    assert!(!tmp_path.exists());

    // 3. Genuine CFX temporary file renamed to .txt (still detected)
    let renamed_path = dir.path().join("renamed_interrupted.txt");
    let mut f = fs::File::create(&renamed_path).unwrap();
    f.write_all(b"CFX~").unwrap();
    f.write_all(&[1, 1, 1]).unwrap();
    f.write_all(&[0u8; 32]).unwrap();
    f.write_all(&[0u8; 8]).unwrap();
    f.sync_all().unwrap();

    cfx()
        .arg("recover")
        .arg(&renamed_path)
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleanup successful."));

    assert!(!renamed_path.exists());
}

#[test]
fn test_recover_scan() {
    let dir = tempdir().unwrap();

    // Create some files
    let junk_path = dir.path().join("junk.tmp");
    fs::write(&junk_path, "junk").unwrap();

    let valid1 = dir.path().join("1.tmp");
    let mut f = fs::File::create(&valid1).unwrap();
    f.write_all(b"CFX~").unwrap();
    f.write_all(&[1, 1, 1]).unwrap();
    f.write_all(&[0u8; 32]).unwrap();
    f.write_all(&[0u8; 8]).unwrap();
    f.sync_all().unwrap();

    let valid2 = dir.path().join("2.bak");
    let mut f = fs::File::create(&valid2).unwrap();
    f.write_all(b"CFX~").unwrap();
    f.write_all(&[1, 1, 1]).unwrap();
    f.write_all(&[0u8; 32]).unwrap();
    f.write_all(&[0u8; 8]).unwrap();
    f.sync_all().unwrap();

    // Scan with --force
    cfx()
        .arg("recover")
        .arg("--scan")
        .arg(dir.path())
        .arg("--force")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Found 2 verified CFX temporary file(s):")
                .and(predicate::str::contains("Cleanup successful.")),
        );

    // Junk is untouched, verified ones are gone
    assert!(junk_path.exists());
    assert!(!valid1.exists());
    assert!(!valid2.exists());
}

#[test]
fn test_recover_errors() {
    // Non-existent path
    cfx()
        .arg("recover")
        .arg("does_not_exist.tmp")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    // Directory provided as exact file
    let dir = tempdir().unwrap();
    cfx()
        .arg("recover")
        .arg(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Not a regular file"));
}
