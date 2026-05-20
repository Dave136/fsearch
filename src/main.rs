mod cli;
mod colour;
mod file;

use std::path::Path;

use clap::Parser;

use crate::file::search;

/// Find based in the current_dir the file to settled in search
/// it should return the result or an empty vec

fn main() {
    let args = cli::Args::parse();

    let root_path = Path::new(&args.root_path);
    let search_str = args.search.as_str();

    if let Ok(results) = search(root_path, search_str) {
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
        let path = Path::new(".");
        let results = search(path, "no_exist").unwrap();

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn should_list_cargo_items() {
        let path = Path::new(".");
        let results = search(path, "Cargo").unwrap();

        assert_eq!(results.len(), 2);
    }
}
