use crate::Args;
use std::{
    io,
    process::{Command, ExitStatus},
};

pub fn checkout_target(target: &String, args: &Args) -> Result<ExitStatus, io::Error> {
    base_git_command(&args)
        .args(["-c", "advice.detachedHead=false"])
        .args(["checkout", target])
        .status()
}

pub fn get_commits(args: &Args) -> Result<Vec<String>, io::Error> {
    let output = base_git_command(&args)
        .arg("rev-list")
        .arg(&args.target)
        .arg("--reverse")
        .output()?;

    let raw = String::from_utf8_lossy(&output.stdout);

    Ok(raw.lines().map(|s| s.to_string()).collect())
}

fn base_git_command(args: &Args) -> Command {
    let mut command = Command::new("git");

    if let Some(path) = &args.directory {
        command.args(["-C", path.to_str().expect("invalid path")]);
    }

    command
}
