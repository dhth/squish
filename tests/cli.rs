use assert_cmd::cargo::CargoError;
use assert_cmd::Command;

// SUCCESSES
fn command_with_default_usage() -> Result<Command, CargoError> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("tests/input.png");
    cmd.arg("-o=/var/tmp/output.png");
    Ok(cmd)
}

#[test]
fn cli_shows_help() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    assert!(stdout.contains("squish lets you resize images via the command line"));
}

#[test]
fn cli_works_for_local_file() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
}

#[test]
fn cli_works_when_width_provided() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
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
fn cli_works_when_blurring_requested() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
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
fn cli_works_when_verbose_output_requested() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
    cmd.arg("-v");
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"input image is 1136 px wide and 668 px tall, and is of the format PNG
resized image is 800 px wide and 470 px tall; size reduced by 50.45%
written to "/var/tmp/output.png"
"#;
    assert_eq!(stdout, expected);
}

#[test]
fn cli_works_when_markdown_address_requested() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
    cmd.arg("-m");
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(stdout, "![image](/var/tmp/output.png)\n");
}

#[test]
fn cli_works_when_several_flags_are_provided() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
    cmd.arg("-w=400");
    cmd.arg("-b=4");
    cmd.arg("-v");
    cmd.arg("-m");
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"input image is 1136 px wide and 668 px tall, and is of the format PNG
resized image is 400 px wide and 235 px tall; size reduced by 87.61%
written to "/var/tmp/output.png"
![image](/var/tmp/output.png)
"#;
    assert_eq!(stdout, expected);
}

// FAILURES

#[test]
fn cli_fails_if_file_non_existent() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("tests/absent.png");
    cmd.arg("-o=/var/tmp/output.png");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("Error: couldn't fetch file metadata"));
}

#[test]
fn cli_fails_if_incorrect_file_provided() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("Cargo.toml");
    cmd.arg("-o=/var/tmp/output.png");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("Error: couldn't determine the image format"));
}

#[test]
fn cli_fails_when_incorrect_width_provided() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
    let cmd = cmd.arg("-w=blah");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}

#[test]
fn cli_fails_when_incorrect_blur_strength_provided() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
    cmd.arg("-b=blah");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}

#[test]
fn cli_fails_when_negative_blur_strength_provided() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
    cmd.arg("-b=-1");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}

#[test]
fn cli_fails_when_blur_strength_exceeds_upper_threshold() {
    // GIVEN
    // WHEN
    let mut cmd = command_with_default_usage().unwrap();
    cmd.arg("-b=256");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}
