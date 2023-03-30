use clap::Parser;
use std::{
    fs::{self},
    path::{PathBuf},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to compile
    #[clap(group = "input")]
    files: Vec<PathBuf>,
}

fn execute(file_path: PathBuf) {
    let contents = fs::read_to_string(file_path).expect(&format!("Failed to open file"));
    println!("With text:\n{}", contents);
}

fn main() {
    let args = Args::parse();

    args.files
        .into_iter()
        .for_each(|file_path| execute(file_path));
}
