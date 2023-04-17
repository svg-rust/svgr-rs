use swc_core::{
    common::DUMMY_SP,
    ecma::ast::*,
};

use super::core;

pub struct TemplateVariables {
    pub component_name: String,
    pub interfaces: Vec<TsInterfaceDecl>,
    pub props: Vec<Pat>,
    pub imports: Vec<ModuleItem>,
    pub exports: Vec<ModuleItem>,
    pub jsx: JSXElement,
}

pub enum JSXRuntime {
    Automatic,
    Classic
}

pub enum ExpandProps {
    Bool(bool),
    Start,
    End,
}

pub enum ExportType {
    Default,
    Named,
}

#[derive(Default)]
pub struct Options {
    pub typescript: bool,
    pub title_prop: bool,
    pub desc_prop: bool,
    pub expand_props: Option<ExpandProps>,
    pub _ref: bool,
    // pub template: Option<Box<dyn Template>>,
    pub native: bool,
    pub memo: bool,
    pub export_type: Option<ExportType>,
    pub named_export: Option<String>,
    pub jsx_runtime: Option<JSXRuntime>,
    pub jsx_runtime_import: Option<core::config::JSXRuntimeImport>,
    pub import_source: Option<String>,
}

pub fn get_variables(opts: Options, state: &core::state::InternalConfig, jsx: JSXElement) -> TemplateVariables {
    let mut interfaces = vec![];
    let mut props = vec![];
    let mut imports = vec![];
    let mut exports = vec![];

    let import_source = opts.import_source.unwrap_or("react".to_string());
    let mut export_identifier = state.component_name.clone();

    if let Some(jsx_runtime) = opts.jsx_runtime {
        let is_automatic = if let JSXRuntime::Automatic = jsx_runtime {
            true
        } else {
            false
        };
        if !is_automatic {
            match opts.jsx_runtime_import {
                Some(jsx_runtime_import) => {
                    imports.push(get_jsx_runtime_import(&jsx_runtime_import));
                }
                None => {
                    let default_jsx_runtime_import = core::config::JSXRuntimeImport {
                        source: "react".to_string(),
                        namespace: Some("React".to_string()),
                        ..Default::default()
                    };
                    imports.push(get_jsx_runtime_import(&default_jsx_runtime_import));
                }
            }
        }
    }

    if opts._ref {
        if props.len() == 0 {
            props.push(Pat::Ident(BindingIdent::from(Ident::new(
                "_".into(),
                DUMMY_SP
            ))));
        }
        let prop = Pat::Ident(BindingIdent::from(Ident::new(
            "ref".into(),
            DUMMY_SP
        )));
        props.push(prop);

        let specifier = ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            local: Ident {
                span: DUMMY_SP,
                sym: "forwardRef".into(),
                optional: false,
            },
            imported: None,
            is_type_only: false,
        });
        get_or_create_import(&mut imports, import_source.as_str(), specifier);

        exports.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(BindingIdent::from(Ident::new(
                    "ForwardRef".into(),
                    DUMMY_SP
                ))),
                definite: false,
                init: Some(Box::new(Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                        "forwardRef".into(),
                        DUMMY_SP
                    )))),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Ident(Ident::new(
                            export_identifier.into(),
                            DUMMY_SP
                        ))),
                    }],
                    type_args: None,
                }))),
            }],
        })))));
        export_identifier = "ForwardRef".to_string();
    }

    exports.push(ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(ExportDefaultExpr {
        span: DUMMY_SP,
        expr: Box::new(Expr::Ident(Ident::new(
            export_identifier.into(),
            DUMMY_SP
        ))),
    })));

    TemplateVariables {
        component_name: state.component_name.clone(),
        interfaces,
        props,
        imports,
        exports,
        jsx,
    }
}

fn get_jsx_runtime_import(cfg: &core::config::JSXRuntimeImport) -> ModuleItem {
    let specifiers = get_jsx_runtime_import_specifiers(cfg);

    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers,
        src: Box::new(Str {
            span: DUMMY_SP,
            value: cfg.source.clone().into(),
            raw: None,
        }),
        type_only: false,
        asserts: None,
    }))
}

fn get_jsx_runtime_import_specifiers(cfg: &core::config::JSXRuntimeImport) -> Vec<ImportSpecifier> {
    if let Some(namespace) = cfg.namespace.clone() {
        let specifier = ImportSpecifier::Namespace(ImportStarAsSpecifier {
            span: DUMMY_SP,
            local: Ident {
                span: DUMMY_SP,
                sym: namespace.into(),
                optional: false,
            },
        });
        return vec![specifier];
    }

    if let Some(default_specifier) = cfg.default_specifier.clone() {
        let specifier = ImportSpecifier::Default(ImportDefaultSpecifier {
            span: DUMMY_SP,
            local: Ident {
                span: DUMMY_SP,
                sym: default_specifier.into(),
                optional: false,
            },
        });
        return vec![specifier];
    }

    if let Some(specifiers) = cfg.specifiers.clone() {
        let mut import_specifiers = vec![];
        for specifier in specifiers {
            import_specifiers.push(ImportSpecifier::Named(ImportNamedSpecifier {
                span: DUMMY_SP,
                local: Ident {
                    span: DUMMY_SP,
                    sym: specifier.into(),
                    optional: false,
                },
                imported: None,
                is_type_only: false,
            }));
        }
        return import_specifiers;
    }

    panic!(r#"Specify "namespace", "defaultSpecifier", or "specifiers" in "jsxRuntimeImport" option"#);
}

fn get_or_create_import(imports: &mut Vec<ModuleItem>, soruce_value: &str, specifier: ImportSpecifier) {
    let mut existing = None;
    for import in imports.iter_mut() {
        if let ModuleItem::ModuleDecl(module_decl) = import {
            if let ModuleDecl::Import(import_decl) = module_decl {
                let is_namespace_import = import_decl.specifiers.iter().any(|specifier| {
                    if let ImportSpecifier::Namespace(_) = specifier {
                        true
                    } else {
                        false
                    }
                });
                if !is_namespace_import && import_decl.src.value.to_string() == soruce_value {
                    existing = Some(import_decl);
                    break;
                }
            }
        }
    }

    if let Some(import_decl) = existing {
        import_decl.specifiers.push(specifier);
        return;
    }

    let module_item = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![specifier],
        src: Box::new(Str {
            span: DUMMY_SP,
            value: soruce_value.into(),
            raw: None,
        }),
        type_only: false,
        asserts: None,
    }));
    imports.push(module_item);
}
