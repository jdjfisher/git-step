use clap::{Parser, ValueEnum};
use console::{Key, Term};
use std::{io, path::PathBuf};

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

    /// Verbose
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum HeadMode {
    Target,
    Original,
    Final,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let term = Term::stdout();

    let commits = git::get_commits(&args);
    let mut index = commits.len() - 1;
    let mut steps = 1;

    move_head(&term, &args, &args.target)?;

    // TODO: Tidy
    loop {
        if let Ok(character) = term.read_key() {
            match character {
                Key::ArrowLeft => {
                    if index > 1 {
                        index -= 1;
                        move_head(&term, &args, &commits[index])?;
                        steps += 1;
                    }
                }
                Key::ArrowRight => {
                    if index < commits.len() - 1 {
                        index += 1;
                        move_head(&term, &args, &commits[index])?;
                        steps += 1;
                    }
                }
                _ => break,
            }
        }
    }

    reset_head(&term, &args, steps)?;

    Ok(())
}

fn move_head(term: &Term, args: &Args, head: &String) -> Result<(), io::Error> {
    term.clear_screen()?;
    git::checkout_target(head, args);
    Ok(())
}

fn reset_head(term: &Term, args: &Args, steps: i32) -> Result<(), io::Error> {
    match args.head {
        HeadMode::Target => move_head(term, args, &args.target),
        HeadMode::Original => move_head(term, args, &format!("@{{-{}}}", steps)),
        HeadMode::Final => Ok(()),
    }
}
