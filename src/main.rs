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

    move_head(&term, &args, &args.target)?;
    let mut steps = 1;

    let loop_result = step_loop(&term, &args, &mut steps);

    // Always try to reset the head after breaking the loop
    reset_head(&term, &args, steps)?;

    loop_result
}

fn move_head(term: &Term, args: &Args, head: &String) -> Result<(), io::Error> {
    term.clear_screen()?;
    git::checkout_target(head, args)?; // TODO: Check the exit status?
    Ok(())
}

fn step_loop(term: &Term, args: &Args, steps: &mut i32) -> Result<(), io::Error> {
    let commits = git::get_commits(&args)?;
    let mut index = commits.len() - 1;

    loop {
        if let Ok(character) = term.read_key() {
            match character {
                Key::ArrowLeft => {
                    if index > 1 {
                        index -= 1;
                        move_head(&term, &args, &commits[index])?;
                        *steps += 1;
                    }
                }
                Key::ArrowRight => {
                    if index < commits.len() - 1 {
                        index += 1;
                        move_head(&term, &args, &commits[index])?;
                        *steps += 1;
                    }
                }
                _ => break,
            }
        }
    }

    Ok(())
}

fn reset_head(term: &Term, args: &Args, steps: i32) -> Result<(), io::Error> {
    match args.head {
        HeadMode::Target => move_head(term, args, &args.target),
        HeadMode::Original => move_head(term, args, &format!("@{{-{}}}", steps)),
        HeadMode::Final => Ok(()),
    }
}
