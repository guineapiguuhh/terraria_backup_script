use std::{
    env::{consts::OS, current_dir},
    io,
};

use crate::backup::Step;
use console::Term;
use indicatif::TermLike;

pub mod backup;
pub mod file_utils;
pub mod terraria;

fn main() -> io::Result<()> {
    let term = Term::stdout();
    let term_err = Term::stderr();

    if OS != "windows" {
        term.write_line("This CLI program has only been tested on Windows!")?;
    }

    let dir = match current_dir() {
        Ok(x) => x,
        Err(err) => {
            term_err.write_line("Failed to get the current directory")?;
            return Err(err);
        }
    };
    let dir = match dir.to_str() {
        Some(x) => x,
        None => return term_err.write_line("dir.to_str returned None"),
    };
    term.flush()?;

    let backup_dir = backup::create_backup_folders(dir);
    term.write_line(&format!("created\n -> {}\n", backup_dir))?;
    term.flush()?;

    let steps = Step::all_steps();

    term.write_line("copied")?;
    match backup::copy_to_backup_folder(&backup_dir, &steps, |step| {
        let _ = term.write_line(&format!(" -> {:?}", step));
        let _ = term.flush();
    }) {
        Ok(_) => Ok(()),
        Err(err) => {
            term_err.write_line("Backup failure")?;
            Err(err)
        }
    }
}
