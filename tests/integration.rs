use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::{prelude::*, str::contains};
use rexpect::process::wait::WaitStatus;

use crate::common::GitStep;

mod common;

#[test]
fn displays_help_prompt() {
    let temp_dir = TempDir::new().unwrap();

    GitStep::make(&temp_dir)
        .arg("--help")
        .non_interactive()
        .success()
        .stdout(predicate::str::contains(
            "Usage: git-step [OPTIONS] [TARGET]",
        ));
}

#[test]
fn propagates_missing_git_repository_error() {
    let temp_dir = TempDir::new().unwrap();

    temp_dir.child(".git").assert(predicate::path::missing());

    GitStep::make(&temp_dir)
        .non_interactive()
        .failure()
        .code(1)
        .stderr(contains(
            "fatal: not a git repository (or any of the parent directories): .git",
        ));
}

#[test]
fn propagates_invalid_git_target_error() {
    let temp_dir = common::setup_temp_git_repository();

    GitStep::make(&temp_dir)
        .arg("lorem")
        .non_interactive()
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

    let mut session = GitStep::make(&temp_dir).arg("@~1").interactive();

    session.exp_regex("HEAD is now at \\w{7} edited").unwrap();

    temp_dir.child("README.md").assert("Foo bar");

    session.send("q").unwrap();
    session.flush().unwrap();

    let status = session.process.wait().unwrap();
    assert!(matches!(status, WaitStatus::Exited(_, 0)));
}

#[test]
fn it_can_step_the_head_back_and_exit() {
    let temp_dir = common::setup_temp_git_repository();

    temp_dir.child("README.md").assert("Fizz buzz");

    let mut session = GitStep::make(&temp_dir).interactive();

    session.send("a").unwrap();
    session.flush().unwrap();

    session.exp_regex("HEAD is now at \\w{7} edited").unwrap();

    session.send("q").unwrap();
    session.flush().unwrap();

    let status = session.process.wait().unwrap();
    assert!(matches!(status, WaitStatus::Exited(_, 0)));

    temp_dir.child("README.md").assert("Foo bar");
}
