use assert_fs::{
    prelude::{FileWriteStr, PathAssert, PathChild},
    TempDir,
};
use std::process::Command;

fn run_git_command(dir: &TempDir, subcommand: &str, args: Vec<&str>) {
    let output = Command::new("git")
        .current_dir(&dir)
        .arg(subcommand)
        .args(args)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

// TODO: Tidy, builder pattern?
pub fn setup_temp_git_repository() -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    run_git_command(&temp_dir, "init", vec![]);

    temp_dir.child(".git").assert(predicates::path::exists());

    temp_dir
        .child("README.md")
        .write_str("Hello world")
        .unwrap();

    run_git_command(&temp_dir, "add", vec!["."]);
    run_git_command(&temp_dir, "commit", vec!["-m", "init"]);

    temp_dir.child("README.md").write_str("Foo bar").unwrap();

    run_git_command(&temp_dir, "add", vec!["."]);
    run_git_command(&temp_dir, "commit", vec!["-m", "edited"]);

    temp_dir.child("README.md").write_str("Fizz buzz").unwrap();

    run_git_command(&temp_dir, "add", vec!["."]);
    run_git_command(&temp_dir, "commit", vec!["-m", "edited again"]);

    temp_dir
}
