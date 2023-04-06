use clap::Parser;
use std::{
    path::{PathBuf}, sync::Arc, borrow::Borrow,
};
use swc_common::{SourceMap, DUMMY_SP};
use swc_ecmascript::ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_xml::{self, visit::VisitWith};

mod visitor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files to compile
    #[clap(group = "input")]
    files: Vec<PathBuf>,
}

fn execute(input: PathBuf) {
    let cm = Arc::<SourceMap>::default();

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

    let mut body = vec![];

    body.push(ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![
            ImportSpecifier::Namespace(ImportStarAsSpecifier {
                span: DUMMY_SP,
                local: Ident {
                    span: DUMMY_SP,
                    sym: "React".into(),
                    optional: false,
                },
            }),
        ],
        src: Box::new(Str {
            span: DUMMY_SP,
            value: "react".into(),
            raw: None,
        }),
        type_only: false,
        asserts: None,
    })));

    document.visit_with(&mut visitor::SvgToReactAst {
        body: &mut body,
    });

    body.push(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(ExportDefaultExpr {
        span: DUMMY_SP,
        expr: Box::new(Expr::Ident(Ident::new("SVG".into(), DUMMY_SP))),
    })));

    let m = Module {
        span: DUMMY_SP,
        body,
        shebang: None,
    };

    let code = {
        let mut buf = vec![];

        {
            let mut emitter = Emitter {
                cfg: swc_ecma_codegen::Config {
                    ..Default::default()
                },
                cm: cm.clone(),
                comments: None,
                wr: JsWriter::new(cm, "\n", &mut buf, None),
            };

            emitter.emit_module(&m).unwrap();
        }

        String::from_utf8_lossy(&buf).to_string()
    };

    println!("{}", code);
}

fn main() {
    let args = Args::parse();

    args.files
        .into_iter()
        .for_each(|input| execute(input));
}
