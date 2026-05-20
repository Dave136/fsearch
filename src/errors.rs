use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SearchErrors {
    CurrentDirectoryError,
}

impl fmt::Display for SearchErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match *self {
            SearchErrors::CurrentDirectoryError => "Cannot access to current directory",
        };
        f.write_str(description)
    }
}

impl Error for SearchErrors {}

pub type SearchResult<T> = Result<T, SearchErrors>;
