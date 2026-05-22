use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Root path to find
    pub root_path: String,

    /// File to search
    pub search: String,

    /// Search by case insensitive
    #[arg(short)]
    pub insensitive: bool,
}
