use clap::Parser;
use std::{
    path::{PathBuf}, sync::Arc, borrow::Borrow,
};
use swc_common::{SourceMap, DUMMY_SP};
use swc_ecmascript::ast::{ExprStmt, Expr, Ident, Module, ModuleItem, Stmt, JSXElement, JSXOpeningElement, JSXElementName};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_xml::{self, visit::VisitWith};

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

    struct SvgRVisitor<'a> {
        body: &'a mut Vec<ModuleItem>,
    }

    impl swc_xml::visit::Visit for SvgRVisitor<'_> {
        fn visit_element(&mut self, n: &swc_xml::ast::Element) {
            println!("{:?}",  n.tag_name);

            let opening = JSXOpeningElement {
                span: DUMMY_SP,
                name: JSXElementName::Ident(Ident::new("a".into(), DUMMY_SP)),
                attrs: vec![],
                self_closing: true,
                type_args: None,
            };

            let element = JSXElement {
                span: DUMMY_SP,
                opening,
                children: vec![],
                closing: None,
            };

            let stmt = ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::JSXElement(Box::new(element)))
            };

            self.body.push(ModuleItem::Stmt(Stmt::Expr(stmt)));
        }
    }

    document.visit_with(&mut SvgRVisitor {
        body: &mut body,
    });

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
