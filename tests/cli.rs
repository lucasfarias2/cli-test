use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("packlify-cli").unwrap();
    cmd.assert().success();
}