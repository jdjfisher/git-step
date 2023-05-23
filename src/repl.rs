use std::io::Write;

use anyhow::Result;
use console::{Key, Style, Term};
use once_cell::sync::Lazy;

use crate::{git, Args};

struct Action {
    keybind: char,
    description: String,
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Styles {
    title: Style,
    primary: Style,
    secondary: Style,
}

static STYLES: Lazy<Styles> = Lazy::new(|| Styles {
    title: Style::new().cyan().bold(),
    primary: Style::new().white(),
    secondary: Style::new().dim(),
});

static ACTIONS: Lazy<[Action; 5]> = Lazy::new(|| {
    [
        Action::new('h', "show help"),
        Action::new('a', "step back"),
        Action::new('d', "step forward"),
        Action::new('c', "clear console"),
        Action::new('q', "exit"),
    ]
});

impl Action {
    fn new(keybind: char, description: &str) -> Self {
        Self {
            keybind,
            description: description.to_string(),
        }
    }

    fn get_prompt(&self) -> String {
        format_args!(
            "{} {} {}",
            STYLES.secondary.apply_to("press"),
            STYLES.primary.apply_to(self.keybind),
            STYLES
                .secondary
                .apply_to(format!("to {}", self.description))
        )
        .to_string()
    }
}

pub fn start(term: &mut Term, args: &Args, steps: &mut i32) -> Result<()> {
    let commits = git::get_commits(&args)?;
    let mut index = commits.len() - 1;

    display_menu(term, args)?;

    loop {
        if let Ok(character) = term.read_key() {
            match character {
                Key::Char('a') | Key::ArrowLeft => {
                    if index > 1 {
                        index -= 1;
                        git::checkout_target(&commits[index], args)?;
                        *steps += 1;
                    }
                }
                Key::Char('d') | Key::ArrowRight => {
                    if index < commits.len() - 1 {
                        index += 1;
                        git::checkout_target(&commits[index], args)?;
                        *steps += 1;
                    }
                }
                Key::Char('h') => display_menu(term, args)?,
                Key::Char('c') => term.clear_screen()?,
                Key::Char('q') => break,
                _ => continue,
            }
        }
    }

    Ok(())
}

fn display_menu(term: &mut Term, args: &Args) -> Result<()> {
    term.clear_screen()?;

    let head = git::parse_head(args)?;

    term.write_fmt(format_args!(
        "{} {}\n\n",
        STYLES.title.apply_to([NAME, VERSION].join(" ")),
        STYLES.secondary.apply_to(head),
    ))?;

    for action in ACTIONS.iter() {
        term.write_line(action.get_prompt().as_str())?
    }

    term.write_line("")?;

    Ok(())
}
