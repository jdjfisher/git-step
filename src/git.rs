use crate::Args;
use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::process::{Command, Stdio};

pub fn checkout_target(target: &String, args: &Args) -> Result<()> {
    let status = base_git_command(&args)
        .args(["-c", "advice.detachedHead=false"])
        .args(["checkout", target])
        .stdout(Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("checkout failed");
    }

    Ok(())
}

pub fn get_commits(args: &Args) -> Result<Vec<String>> {
    let output = base_git_command(&args)
        .arg("rev-list")
        .arg(&args.target)
        .arg("--reverse")
        .output()?;

    if !output.status.success() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        bail!("failed to load commits");
    }

    let raw = String::from_utf8_lossy(&output.stdout);

    Ok(raw.lines().map(|s| s.to_string()).collect())
}

pub fn parse_head(args: &Args) -> Result<String> {
    let output = base_git_command(&args).arg("branch").output()?;

    if !output.status.success() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        bail!("failed to parse head");
    }

    let pattern = Regex::new(r"\* (.*)").unwrap();

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .find_map(|line| {
            pattern
                .captures(line)
                .map(|captures| captures[1].to_string())
        })
        .ok_or(anyhow!("failed to parse head"))
}

fn base_git_command(args: &Args) -> Command {
    let mut command = Command::new("git");

    if let Some(path) = &args.path {
        command.args(["-C", path.to_str().expect("invalid path")]);
    }

    command
}
