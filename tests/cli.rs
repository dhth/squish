use std::process::Command;

use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};

fn base_command() -> Command {
    Command::new(get_cargo_bin("squish"))
}

// SUCCESSES

#[test]
fn cli_shows_help() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args(["--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    squish lets you resize images via the command line

    Usage: squish [OPTIONS] <INPUT>

    Arguments:
      <INPUT>  Local file path, or "cb" for system clipboard

    Options:
      -w, --width <INTEGER>          Width of resized image [default: 800]
      -o, --output-file <FILE>       Destination of resized output file
      -c, --copy-to-clipboard        Whether to copy resized image to clipboard (only supported for PNG images)
      -b, --blur-strength <INTEGER>  Blur strength [default: 0]
      -v, --verbose                  Whether to print updates
      -m, --print-markdown-address   Whether to print address of output file in markdown format
      -h, --help                     Print help

    ----- stderr -----
    "#);
}

#[test]
fn cli_works_for_local_file() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args(["tests/input.png", "--output-file", "/var/tmp/output.png"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");
}

#[test]
fn cli_works_when_width_provided() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args([
        "tests/input.png",
        "--output-file",
        "/var/tmp/output.png",
        "--width",
        "600",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");
}

#[test]
fn cli_works_when_blurring_requested() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args([
        "tests/input.png",
        "--output-file",
        "/var/tmp/output.png",
        "--blur-strength",
        "8",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");
}

#[test]
fn cli_works_when_verbose_output_requested() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args([
        "tests/input.png",
        "--output-file",
        "/var/tmp/output.png",
        "--verbose",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    input image is 1136 px wide and 668 px tall, and is of the format PNG
    resized image is 800 px wide and 470 px tall; size reduced by 50.45%
    written to "/var/tmp/output.png"

    ----- stderr -----
    "#);
}

#[test]
fn cli_works_when_markdown_address_requested() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args([
        "tests/input.png",
        "--output-file",
        "/var/tmp/output.png",
        "--print-markdown-address",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ![image](/var/tmp/output.png)

    ----- stderr -----
    ");
}

#[test]
fn cli_works_when_several_flags_are_provided() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args([
        "tests/input.png",
        "--output-file",
        "/var/tmp/output.png",
        "--width",
        "400",
        "--blur-strength",
        "4",
        "--verbose",
        "--print-markdown-address",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    input image is 1136 px wide and 668 px tall, and is of the format PNG
    resized image is 400 px wide and 235 px tall; size reduced by 87.61%
    written to "/var/tmp/output.png"
    ![image](/var/tmp/output.png)

    ----- stderr -----
    "#);
}

// FAILURES

#[test]
fn cli_fails_if_file_non_existent() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args(["tests/absent.png", "--output-file", "/var/tmp/output.png"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't fetch file metadata

    Caused by:
        No such file or directory (os error 2)
    ");
}

#[test]
fn cli_fails_if_incorrect_file_provided() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args(["Cargo.toml", "--output-file", "/var/tmp/output.png"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't determine the image format
    ");
}

#[test]
fn cli_fails_when_incorrect_width_provided() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args([
        "tests/input.png",
        "--output-file",
        "/var/tmp/output.png",
        "--width",
        "blah",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value 'blah' for '--width <INTEGER>': invalid digit found in string

    For more information, try '--help'.
    ");
}

#[test]
fn cli_fails_when_incorrect_blur_strength_provided() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args([
        "tests/input.png",
        "--output-file",
        "/var/tmp/output.png",
        "--blur-strength",
        "blah",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value 'blah' for '--blur-strength <INTEGER>': invalid digit found in string

    For more information, try '--help'.
    ");
}

#[test]
fn cli_fails_when_negative_blur_strength_provided() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args([
        "tests/input.png",
        "--output-file",
        "/var/tmp/output.png",
        "--blur-strength",
        "-1",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: unexpected argument '-1' found

      tip: to pass '-1' as a value, use '-- -1'

    Usage: squish [OPTIONS] <INPUT>

    For more information, try '--help'.
    ");
}

#[test]
fn cli_fails_when_blur_strength_exceeds_upper_threshold() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args([
        "tests/input.png",
        "--output-file",
        "/var/tmp/output.png",
        "--blur-strength",
        "256",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value '256' for '--blur-strength <INTEGER>': 256 is not in 0..=255

    For more information, try '--help'.
    ");
}
