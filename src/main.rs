use anyhow::Result;
use clap::Parser;
use console::{Key, Term};
use std::path::PathBuf;

pub mod git;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target branch or commit to step back from
    #[arg(default_value_t = String::from("HEAD"))]
    target: String,

    /// Reset HEAD to original position on exit
    #[arg(short, long)]
    reset_head: bool,

    /// Run as if git was started in <path>
    #[arg(short = 'C')]
    path: Option<PathBuf>,

    /// Verbose
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let term = Term::stdout();

    let mut steps = 0;

    // Move head to the target
    if args.target != "HEAD" {
        move_head(&term, &args, &args.target)?;
        steps += 1;
    }

    // Step...
    let loop_result = step_loop(&term, &args, &mut steps);

    // Try resetting the head regardless of loop Result
    if args.reset_head {
        reset_head(&term, &args, steps)?;
    }

    loop_result
}

fn move_head(term: &Term, args: &Args, head: &String) -> Result<()> {
    term.clear_screen()?;
    git::checkout_target(head, args)
}

fn step_loop(term: &Term, args: &Args, steps: &mut i32) -> Result<()> {
    let commits = git::get_commits(&args)?;
    let mut index = commits.len() - 1;

    loop {
        if let Ok(character) = term.read_key() {
            match character {
                Key::Char('a') | Key::ArrowLeft => {
                    if index > 1 {
                        index -= 1;
                        move_head(&term, &args, &commits[index])?;
                        *steps += 1;
                    }
                }
                Key::Char('d') | Key::ArrowRight => {
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

fn reset_head(term: &Term, args: &Args, steps: i32) -> Result<()> {
    let reflog_id = format!("@{{-{}}}", steps);

    move_head(term, args, &reflog_id)
}
