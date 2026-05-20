use crate::colour::{Colour, paint};

use std::fs;
use std::{io, path::Path};

const IGNORE_LIST: [&str; 2] = ["target", "node_modules"];

fn has_hidden_files(path: &Path) -> bool {
    if let Some(file) = path.file_name() {
        if let Some(file) = file.to_str() {
            if file.starts_with(".") {
                return true;
            }
        }
    }

    false
}

fn should_ignore(path: &Path) -> bool {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if IGNORE_LIST.contains(&file_name) {
            return true;
        }
    }

    false
}

fn get_files(current_dir: &Path, search: &str, results: &mut Vec<String>) -> io::Result<()> {
    for dir in fs::read_dir(current_dir)? {
        let directory = dir?;
        let dir_entry_path = directory.path();

        if has_hidden_files(&dir_entry_path) {
            continue;
        }

        if dir_entry_path.is_dir() && should_ignore(&dir_entry_path) {
            continue;
        }

        if dir_entry_path.is_dir() && !should_ignore(&dir_entry_path) {
            get_files(&dir_entry_path, search, results)?;
            continue;
        }

        if dir_entry_path.is_file() {
            if let Some(file_name) = dir_entry_path.file_name().and_then(|n| n.to_str()) {
                if file_name.contains(search) {
                    let highlight = paint(Colour::Red, &search);
                    if let Some(display_path) = dir_entry_path.to_str() {
                        results.push(display_path.replace(search, &highlight));
                    }
                    continue;
                }
            }
        }
    }

    Ok(())
}

pub fn search(current_dir: &Path, item: &str) -> io::Result<Vec<String>> {
    let mut results: Vec<String> = Vec::new();

    get_files(current_dir, item, &mut results)?;

    Ok(results)
}
