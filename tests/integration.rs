use assert_cmd::prelude::*;
use assert_fs::{
    prelude::{PathAssert, PathChild},
    TempDir,
};
use predicates::{prelude::*, str::contains};
use std::process::Command;

#[test]
fn displays_help_prompt() {
    Command::cargo_bin("git-step")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Usage: git-step [OPTIONS] <TARGET>",
        ));
}

#[test]
fn propagates_missing_git_repository_error() {
    let temp_dir = TempDir::new().unwrap();

    temp_dir.child(".git").assert(predicate::path::missing());

    Command::cargo_bin("git-step")
        .unwrap()
        .arg("main")
        .args(["-d", temp_dir.to_str().unwrap()])
        .assert()
        .failure()
        .code(1)
        .stderr(contains(
            "fatal: not a git repository (or any of the parent directories): .git",
        ));
}
