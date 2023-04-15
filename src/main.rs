use clap::{Parser, ValueEnum};
use console::{Key, Term};
use std::path::PathBuf;

pub mod git;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target branch or commit to step back from
    target: String,

    /// Mode for resetting the HEAD on exit
    #[arg(long, value_enum, default_value_t = HeadMode::Original)]
    head: HeadMode,

    /// Path
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum HeadMode {
    Target,
    Original,
    Final,
}

fn main() {
    let args = Args::parse();
    let stdout = Term::buffered_stdout();

    let commits = git::get_commits(&args);
    let mut index = commits.len() - 1;
    let mut steps = 1;

    git::checkout_target(&args.target, &args);

    // TODO: Tidy
    loop {
        if let Ok(character) = stdout.read_key() {
            match character {
                Key::ArrowLeft => {
                    if index > 1 {
                        index -= 1;
                        git::checkout_target(&commits[index], &args);
                        steps += 1;
                    }
                }
                Key::ArrowRight => {
                    if index < commits.len() - 1 {
                        index += 1;
                        git::checkout_target(&commits[index], &args);
                        steps += 1;
                    }
                }
                _ => break,
            }
        }
    }

    reset_head(args, steps);
}

fn reset_head(args: Args, steps: i32) {
    match args.head {
        HeadMode::Target => {
            git::checkout_target(&args.target, &args);
        }
        HeadMode::Original => {
            git::checkout_target(&format!("@{{-{}}}", steps), &args);
        }
        HeadMode::Final => {}
    }
}
