use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;

#[test]
fn cannot_run_cli_init_without_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("moose-cli")?;

    cmd.arg("init");
    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided:",
    ));

    Ok(())
}

#[test]
fn can_run_cli_init() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir: &str = temp.path().to_str().unwrap();

    // List the content of dir
    temp.child("package.json")
        .assert(predicate::path::missing());
    temp.child("app").assert(predicate::path::missing());
    temp.child("project.toml")
        .assert(predicate::path::missing());

    let mut cmd = Command::cargo_bin("moose-cli")?;

    cmd.arg("init").arg("test-app").arg("ts").arg(dir);

    cmd.assert().success();

    // TODO add more specific tests when the layout of the
    // app is more stable
    temp.child("package.json").assert(predicate::path::exists());
    temp.child("app").assert(predicate::path::exists());
    temp.child("project.toml").assert(predicate::path::exists());

    Ok(())
}
