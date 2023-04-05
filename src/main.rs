use clap::Parser;
use std::{
    path::{PathBuf}, sync::Arc, borrow::Borrow,
};
use swc_common;
use swc_xml;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to compile
    #[clap(group = "input")]
    files: Vec<PathBuf>,
}

fn execute(input: PathBuf) {
    let cm = Arc::<swc_common::SourceMap>::default();

    let fm = cm
        .load_file(input.borrow())
        .expect(&format!("{} does not exist", input.display()));

    let mut errors = vec![];
    let document = swc_xml::parser::parse_file_as_document(
        fm.borrow(),
        swc_xml::parser::parser::ParserConfig{
            ..Default::default()
        },
        &mut errors
    ).unwrap();

    // for err in &errors {
    //     err.to_diagnostics(&handler).emit();
    // }
}

fn main() {
    let args = Args::parse();

    args.files
        .into_iter()
        .for_each(|input| execute(input));
}
