use crate::colour::{Colour, paint};

use std::fs;
use std::{io, path::Path};

const IGNORE_LIST: [&str; 2] = ["target", "node_modules"];

pub struct File {
    /// Flag to indicate search case sensitive
    insensitive: bool,
}

impl File {
    pub fn new(insensitive: bool) -> Self {
        Self { insensitive }
    }

    fn is_hidden(path: &Path) -> bool {
        if let Some(file) = path.file_name() {
            let file = file.to_string_lossy().to_string();
            if file.starts_with(".") {
                return true;
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

    fn match_indices() {
        todo!("")
    }

    fn get_files(
        &self,
        current_dir: &Path,
        search: &str,
        results: &mut Vec<String>,
    ) -> io::Result<()> {
        for dir in fs::read_dir(current_dir)? {
            let directory = dir?;
            let dir_entry_path = directory.path();

            if Self::is_hidden(&dir_entry_path) {
                continue;
            }

            if dir_entry_path.is_dir() {
                if Self::should_ignore(&dir_entry_path) {
                    continue;
                };
                self.get_files(&dir_entry_path, search, results)?;
                continue;
            }

            if dir_entry_path.is_file() {
                if let Some(file_name) = dir_entry_path.file_name().and_then(|n| n.to_str()) {
                    if self.insensitive
                        && file_name
                            .to_lowercase()
                            .contains(search.to_lowercase().as_str())
                    {
                        let search = search.to_lowercase();
                        let display_path = dir_entry_path.to_string_lossy().to_string();
                        let path_temp = display_path.to_lowercase();
                        let index = path_temp
                            .match_indices(&search)
                            .collect::<Vec<(usize, &str)>>();

                        let mut final_text = String::new();
                        let mut cursor = 0;

                        for item in index {
                            let before_text = &display_path[cursor..item.0];
                            final_text.push_str(before_text);

                            let highlight_slice = &display_path[item.0..(item.0 + search.len())];
                            let highlight = paint(Colour::Red, highlight_slice);

                            final_text.push_str(&highlight);
                            cursor = item.0 + search.len();
                        }

                        final_text.push_str(&display_path[cursor..]);

                        results.push(final_text);
                        continue;
                    }

                    if file_name.contains(search) {
                        let highlight = paint(Colour::Red, search);
                        if let Some(display_path) = dir_entry_path.to_str() {
                            results.push(display_path.replace(search, &highlight));
                        }
                        continue;
                    }
                };
            }
        }

        Ok(())
    }

    pub fn search(&self, current_dir: &Path, item: &str) -> io::Result<Vec<String>> {
        let mut results: Vec<String> = Vec::new();

        self.get_files(current_dir, item, &mut results)?;

        Ok(results)
    }
}
