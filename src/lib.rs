#![feature(path_file_prefix)]

#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::{sync::Arc, borrow::Borrow};
use swc_common::{SourceMap, DUMMY_SP, FileName};
use swc_ecmascript::{ast::*, codegen::{text_writer::JsWriter, Emitter, Config}};
use swc_xml::{parser::{parse_file_as_document, parser}};
use napi::bindgen_prelude::*;

mod hast_to_swc_ast;
pub mod core;

#[napi]
pub async fn transform(code: String, config: Option<core::config::Config>, state: Option<core::state::Config>) -> Result<String> {
    let cm = Arc::<SourceMap>::default();

    let fm = cm.new_source_file(FileName::Anon, code.to_string());

    let mut errors = vec![];
    let document = parse_file_as_document(
        fm.borrow(),
        parser::ParserConfig{
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

    if let Some(expr) = hast_to_swc_ast::to_swc_ast(document) {
        body.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(BindingIdent::from(Ident::new("SVG".into(), DUMMY_SP))),
                definite: false,
                init: Some(Box::new(Expr::Arrow(ArrowExpr {
                    span: DUMMY_SP,
                    params: vec![Pat::Ident(BindingIdent::from(Ident::new("props".into(), DUMMY_SP)))],
                    body: Box::new(BlockStmtOrExpr::Expr(Box::new(Expr::JSXElement(Box::new(expr))))),
                    is_async: false,
                    is_generator: false,
                    type_params: None,
                    return_type: None,
                }))),
            }],
        })))));
    }

    body.push(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(ExportDefaultExpr {
        span: DUMMY_SP,
        expr: Box::new(Expr::Ident(Ident::new("SVG".into(), DUMMY_SP))),
    })));

    let m = Module {
        span: DUMMY_SP,
        body,
        shebang: None,
    };

    let mut buf = vec![];

    let mut emitter = Emitter {
        cfg: Config {
            ..Default::default()
        },
        cm: cm.clone(),
        comments: None,
        wr: JsWriter::new(cm, "\n", &mut buf, None),
    };
    emitter.emit_module(&m).unwrap();

    Ok(String::from_utf8_lossy(&buf).to_string())
}
