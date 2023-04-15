use clap::Parser;
use console::{Key, Term};
use std::path::PathBuf;

pub mod git;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target branch or commit to step back from
    target: String,

    /// Path
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let commits = git::get_commits(&args);

    let stdout = Term::buffered_stdout();

    let mut index = 0;

    loop {
        if let Ok(character) = stdout.read_key() {
            match character {
                Key::ArrowLeft => {
                    if index > 1 {
                        index -= 1;
                        git::checkout_target(&commits[index], &args);
                    }
                }
                Key::ArrowRight => {
                    if index < commits.len() - 1 {
                        index += 1;
                        git::checkout_target(&commits[index], &args);
                    }
                }
                _ => break,
            }
        }
    }
}
