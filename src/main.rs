use console::{Key, Term};

pub mod git;

fn main() {
    let commits = git::get_commits();

    let stdout = Term::buffered_stdout();

    let mut index = 0;

    loop {
        if let Ok(character) = stdout.read_key() {
            match character {
                Key::ArrowLeft => {
                    if index > 1 {
                        index -= 1;
                        git::checkout_commit(&commits[index]);
                    }
                }
                Key::ArrowRight => {
                    if index < commits.len() - 1 {
                        index += 1;
                        git::checkout_commit(&commits[index]);
                    }
                }
                _ => break,
            }
        }
    }
}
