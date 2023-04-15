use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help() {
    Command::cargo_bin("git-step")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Usage: git-step [OPTIONS] <TARGET>",
        ));
}
