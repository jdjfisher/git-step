use std::process::Command;

const DIR: &str = ".";
const HEAD: &str = "main";

pub fn checkout_commit(commit: &String) {
  Command::new("git")
      .arg("-C")
      .arg(DIR)
      .arg("checkout")
      .arg(commit)
      .status()
      .unwrap();
}

pub fn get_commits() -> Vec<String> {
  let output = Command::new("git")
      .arg("-C")
      .arg(DIR)
      .arg("rev-list")
      .arg(HEAD)
      .arg("--reverse")
      .output()
      .unwrap();

  let string = String::from_utf8_lossy(&output.stdout);

  return string.lines().map(|s| s.to_string()).collect();
}