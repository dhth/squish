use assert_cmd::Command;

#[test]
fn shows_help() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin("squish").unwrap();
    cmd.arg("--help");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    assert!(stdout.contains("squish lets you resize images via the command line"));
}

#[test]
fn works_for_local_file() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin("squish").unwrap();
    cmd.arg("tests/input.png");
    cmd.arg("-o=/var/tmp/output.png");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
}

#[test]
fn providing_width_works() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin("squish").unwrap();
    cmd.arg("tests/input.png");
    cmd.arg("-o=/var/tmp/output.png");
    cmd.arg("-w=600");
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
}

#[test]
fn blurring_works() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin("squish").unwrap();
    cmd.arg("tests/input.png");
    cmd.arg("-o=/var/tmp/output.png");
    cmd.arg("-b=8");
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
}

#[test]
fn fails_if_file_non_existent() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin("squish").unwrap();
    cmd.arg("tests/absent.png");
    cmd.arg("-o=/var/tmp/output.png");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("Error: couldn't fetch file metadata"));
}
