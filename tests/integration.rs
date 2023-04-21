use assert_cmd::cargo::cargo_bin;
use assert_cmd::Command;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::{prelude::*, str::contains};
use rexpect::process::wait::WaitStatus;

mod common;

#[test]
fn displays_help_prompt() {
    Command::cargo_bin("git-step")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Usage: git-step [OPTIONS] [TARGET]",
        ));
}

#[test]
fn propagates_missing_git_repository_error() {
    let temp_dir = TempDir::new().unwrap();

    temp_dir.child(".git").assert(predicate::path::missing());

    Command::cargo_bin("git-step")
        .unwrap()
        .args(["-C", temp_dir.to_str().unwrap()])
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "fatal: not a git repository (or any of the parent directories): .git",
        ));
}

#[test]
fn propagates_invalid_git_target_error() {
    let temp_dir = common::setup_temp_git_repository();

    Command::cargo_bin("git-step")
        .unwrap()
        .args(["-C", temp_dir.to_str().unwrap()])
        .arg("lorem")
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "error: pathspec \'lorem\' did not match any file(s) known to git",
        ));
}

#[test]
fn it_moves_head_to_target() {
    let temp_dir = common::setup_temp_git_repository().into_persistent();

    temp_dir.child("README.md").assert("Fizz buzz");

    Command::cargo_bin("git-step")
        .unwrap()
        .arg("@~1")
        .args(["-C", temp_dir.to_str().unwrap()])
        .assert()
        .success();

    temp_dir.child("README.md").assert("Foo bar");
}

#[test]
fn it_can_step_the_head_back_and_exit() {
    let temp_dir = common::setup_temp_git_repository();

    temp_dir.child("README.md").assert("Fizz buzz");

    let command = format!(
        "{} -C {}",
        cargo_bin("git-step").to_str().unwrap(),
        temp_dir.to_str().unwrap()
    );

    let mut session = rexpect::spawn(command.as_str(), Some(10_000)).unwrap();

    session.send("a").unwrap();
    session.send("q").unwrap();
    session.flush().unwrap();

    let status = session.process.wait().unwrap();

    assert!(matches!(status, WaitStatus::Exited(_, 0)));

    temp_dir.child("README.md").assert("Foo bar");
}
