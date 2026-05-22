mod cli;
mod colour;
mod file;

use crate::file::File;
use clap::Parser;
use std::{path::Path, process};

fn main() {
    let args = cli::Args::parse();

    let root_path = Path::new(&args.root_path);
    let search_str = args.search.as_str();

    let file = File::new(args.insensitive);

    if let Ok(results) = file.search(root_path, search_str) {
        if results.is_empty() {
            println!("No results found");
            process::exit(1);
        }

        results
            .iter()
            .for_each(|result| println!("Found => {result}"));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn should_not_return_hidden_files() {
        let file = File::new(true);
        let path = Path::new(".");
        let results = file.search(path, "no_exist").unwrap();

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn should_list_cargo_items() {
        let file = File::new(true);
        let path = Path::new(".");
        let results = file.search(path, "Cargo").unwrap();

        assert_eq!(results.len(), 2);
    }
}
