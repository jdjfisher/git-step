use crate::Args;
use std::process::Command;

pub fn checkout_commit(commit: &String, args: &Args) {
    base_git_command(&args)
        .arg("checkout")
        .arg(commit)
        .status()
        .unwrap();
}

pub fn get_commits(args: &Args) -> Vec<String> {
    let output = base_git_command(&args)
        .arg("rev-list")
        .arg(&args.head)
        .arg("--reverse")
        .output()
        .unwrap();

    let string = String::from_utf8_lossy(&output.stdout);

    return string.lines().map(|s| s.to_string()).collect();
}

fn base_git_command(args: &Args) -> Command {
    let mut command = Command::new("git");

    if let Some(path) = &args.directory {
        command.args(["-C", path.to_str().unwrap()]);
    }

    command
}
