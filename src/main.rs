use std::{process::Command};
use console::{Term, Key};

const DIR: &str = ".";

fn main() {
    let commits = get_commits();
    
    let stdout = Term::buffered_stdout();
    
    let mut index = 0;

    loop {
        if let Ok(character) = stdout.read_key() {
            match character {
                Key::ArrowLeft => {
                    if index > 1 {
                        index -= 1;
                        checkout_commit(&commits[index]);
                    }
                },
                Key::ArrowRight => {
                    if index < commits.len() -1 {
                        index += 1;
                        checkout_commit(&commits[index]);
                    }
                },
                _ => break 
            }
        }
    }
}

fn checkout_commit(commit: &String) {
    Command::new("git")
        .arg("-C")
        .arg(DIR)
        .arg("checkout")
        .arg(commit)
        .status()
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
