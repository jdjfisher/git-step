use std::{fs::read_dir, process::ExitCode};

use assert_cmd::Command;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::{prelude::*, str::contains};

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
        .arg("master")
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "error: pathspec \'master\' did not match any file(s) known to git",
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
