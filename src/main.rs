use std::{process::Command, thread::sleep, time::Duration};

const DIR: &str = ".";

fn main() {
    let commits = get_commits();

    for commit in commits {
        checkout_commit(commit);
        sleep(Duration::from_secs(1));
    }
}

fn checkout_commit(commit: String) {
    Command::new("git")
        .arg("-C")
        .arg(DIR)
        .arg("checkout")
        .arg(commit)
        .spawn()
        .unwrap();
}

fn get_commits() -> Vec<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(DIR)
        .arg("rev-list")
        .arg("main")
        .arg("--reverse")
        .output()
        .unwrap();

    let string = String::from_utf8_lossy(&output.stdout);

    return string.lines().map(|s| s.to_string()).collect();
}
