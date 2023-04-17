use crate::Args;
use std::{
    io,
    process::{Command, Stdio},
};

pub fn checkout_target(target: &String, args: &Args) -> Result<(), io::Error> {
    let status = base_git_command(&args)
        .args(["-c", "advice.detachedHead=false"])
        .args(["checkout", target])
        .stdout(Stdio::inherit())
        .status()?;

    if !status.success() {
        panic!("checkout failed");
    }

    Ok(())
}

pub fn get_commits(args: &Args) -> Result<Vec<String>, io::Error> {
    let output = base_git_command(&args)
        .arg("rev-list")
        .arg(&args.target)
        .arg("--reverse")
        .output()?;

    if !output.status.success() {
        panic!("failed to load commits");
    }

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
