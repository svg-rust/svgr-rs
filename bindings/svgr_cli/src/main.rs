use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to compile
    #[clap(group = "input")]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
