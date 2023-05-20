use anyhow::Result;
use clap::Parser;
use console::Term;
use std::path::PathBuf;

pub mod git;
pub mod repl;

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
    let mut term = Term::stdout();

    let mut steps = 0;

    // Move head to the target
    if args.target != "HEAD" {
        git::checkout_target(&args.target, &args)?;
        steps += 1;
    }

    // Step...
    let repl_result = repl::start(&mut term, &args, &mut steps);

    // Try resetting the head regardless of loop Result
    if args.reset_head {
        reset_head(&args, steps)?;
    }

    repl_result
}

fn reset_head(args: &Args, steps: i32) -> Result<()> {
    let reflog_id = format!("@{{-{}}}", steps);

    git::checkout_target(&reflog_id, args)
}
