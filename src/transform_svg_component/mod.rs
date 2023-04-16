use swc_core::{
    common::DUMMY_SP,
    ecma::ast::*,
};

use crate::core;

pub fn transform(jsx_element: Option<JSXElement>, config: &core::config::Config, state: &core::state::InternalConfig) -> Module {
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

    if let Some(expr) = jsx_element {
        body.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(BindingIdent::from(Ident::new(
                    state.component_name.clone().into(),
                    DUMMY_SP
                ))),
                definite: false,
                init: Some(Box::new(Expr::Arrow(ArrowExpr {
                    span: DUMMY_SP,
                    params: vec![Pat::Ident(BindingIdent::from(Ident::new("props".into(), DUMMY_SP)))],
                    body: Box::new(BlockStmtOrExpr::Expr(Box::new(Expr::Paren(ParenExpr {
                        expr: Box::new(Expr::JSXElement(Box::new(expr))),
                        span: DUMMY_SP,
                    })))),
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
        expr: Box::new(Expr::Ident(Ident::new(
            state.component_name.clone().into(),
            DUMMY_SP
        ))),
    })));

    Module {
        span: DUMMY_SP,
        body,
        shebang: None,
    }
}
