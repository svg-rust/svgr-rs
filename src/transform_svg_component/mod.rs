use swc_core::{
    common::DUMMY_SP,
    ecma::ast::*,
};

use crate::core;

mod variables;

fn get_variables_options(config: &core::config::Config) -> variables::Options {
    let expand_props = match config.expand_props {
        Some(core::config::ExpandProps::Bool(b)) => Some(variables::ExpandProps::Bool(b)),
        Some(core::config::ExpandProps::Start) => Some(variables::ExpandProps::Start),
        Some(core::config::ExpandProps::End) => Some(variables::ExpandProps::End),
        None => None,
    };

    let mut opts = variables::Options {
        typescript: config.typescript.unwrap_or(false),
        title_prop: config.title_prop.unwrap_or(false),
        desc_prop: config.desc_prop.unwrap_or(false),
        expand_props,
        _ref: config._ref.unwrap_or(false),
        native: config.native.unwrap_or(false),
        memo: config.memo.unwrap_or(false),
        ..Default::default()
    };
    
    if let Some(jsx_runtime_import) = config.jsx_runtime_import.clone() {
        opts.import_source = Some(jsx_runtime_import.source.clone());
        opts.jsx_runtime_import = Some(jsx_runtime_import);
        return opts;
    }

    let jsx_runtime = config.jsx_runtime.clone().unwrap_or(core::config::JSXRuntime::Classic);

    match jsx_runtime {
        core::config::JSXRuntime::Classic => {
            opts.jsx_runtime = Some(variables::JSXRuntime::Classic);
            opts.import_source = Some("react".to_string());
            opts.jsx_runtime_import = Some(core::config::JSXRuntimeImport {
                source: "react".to_string(),
                namespace: Some("React".to_string()),
                ..Default::default()
            });
        }
        core::config::JSXRuntime::ClassicPreact => {
            opts.jsx_runtime = Some(variables::JSXRuntime::Classic);
            opts.import_source = Some("preact".to_string());
            opts.jsx_runtime_import = Some(core::config::JSXRuntimeImport {
                source: "preact".to_string(),
                specifiers: Some(vec!["h".to_string()]),
                ..Default::default()
            });
        }
        core::config::JSXRuntime::Automatic => {
            opts.jsx_runtime = Some(variables::JSXRuntime::Automatic);
        }
    }
    
    opts
}

pub fn transform(jsx_element: JSXElement, config: &core::config::Config, state: &core::state::InternalConfig) -> Module {
    let variables_options = get_variables_options(config);

    let variables = variables::get_variables(variables_options, state, jsx_element);

    let mut body = vec![];

    for import in variables.imports {
        body.push(import);
    }

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
                params: variables.props,
                body: Box::new(BlockStmtOrExpr::Expr(Box::new(Expr::JSXElement(Box::new(variables.jsx))))),
                is_async: false,
                is_generator: false,
                type_params: None,
                return_type: None,
            }))),
        }],
    })))));

    for export in variables.exports {
        body.push(export);
    }

    Module {
        span: DUMMY_SP,
        body,
        shebang: None,
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, borrow::Borrow};
    use swc_core::{
        common::{SourceMap, FileName},
        ecma::{parser, codegen::{text_writer::JsWriter, Emitter}},
    };

    use crate::core;

    use super::*;

    fn test_code(input: &str, config: &core::config::Config, state: &core::state::InternalConfig, expected: &str) {
        let cm = Arc::<SourceMap>::default();
        let fm = cm.new_source_file(FileName::Anon, input.to_string());

        let mut recovered_errors = vec![];
        let expr = parser::parse_file_as_expr(
            fm.borrow(),
            parser::Syntax::Es(parser::EsConfig {
                jsx: true,
                ..Default::default()
            }),
            EsVersion::Es2020,
            None,
            &mut recovered_errors
        ).unwrap();

        let jsx_element = expr.as_jsx_element().unwrap();

        let m = transform(*jsx_element.clone(), config, state);

        let mut buf = vec![];
        let mut emitter = Emitter {
            cfg: Default::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm, "\n", &mut buf, None),
        };
        emitter.emit_module(&m).unwrap();
        let result = String::from_utf8_lossy(&buf).to_string();

        assert_eq!(result, expected);
    }

    #[test]
    fn transforms_whole_program() {
        test_code(
            r#"<svg><g/></svg>"#,
            &core::config::Config {
                ..Default::default()
            },
            &core::state::InternalConfig {
                ..Default::default()
            },
            r#"import * as React from "react";
const SvgComponent = ()=><svg><g/></svg>;
export default SvgComponent;
"#
        );
    }

    #[test]
    fn with_native_option_adds_import_from_react_native_svg() {
        test_code(
            r#"<Svg><g/></Svg>"#,
            &core::config::Config {
                native: Some(true),
                ..Default::default()
            },
            &core::state::InternalConfig {
                ..Default::default()
            },
            r#"import * as React from "react";
import Svg from "react-native-svg";
const SvgComponent = ()=><Svg><g/></Svg>;
export default SvgComponent;
"#
        );
    }
    
    #[test]
    fn with_ref_option_adds_forward_ref_component() {
        test_code(
            r#"<svg><g/></svg>"#,
            &core::config::Config {
                _ref: Some(true),
                ..Default::default()
            },
            &core::state::InternalConfig {
                component_name: "SvgComponent".to_string(),
                ..Default::default()
            },
            r#"import * as React from "react";
import { forwardRef } from "react";
const SvgComponent = (_, ref)=><svg><g/></svg>;
const ForwardRef = forwardRef(SvgComponent);
export default ForwardRef;
"#
        );
    }

    #[test]
    fn with_title_prop_adds_title_and_title_id_prop() {
        test_code(
            r#"<svg><g/></svg>"#,
            &core::config::Config {
                title_prop: Some(true),
                ..Default::default()
            },
            &core::state::InternalConfig {
                ..Default::default()
            },
            r#"import * as React from "react";
const SvgComponent = ({ title , titleId  })=><svg><g/></svg>;
export default SvgComponent;
"#
        );
    }

    #[test]
    fn with_title_prop_and_expand_props_adds_title_title_id_props_and_expands_props() {
        test_code(
            r#"<svg><g/></svg>"#,
            &core::config::Config {
                title_prop: Some(true),
                expand_props: Some(core::config::ExpandProps::Bool(true)),
                ..Default::default()
            },
            &core::state::InternalConfig {
                ..Default::default()
            },
            r#"import * as React from "react";
const SvgComponent = ({ title , titleId , ...props })=><svg><g/></svg>;
export default SvgComponent;
"#
        );
    }

    #[test]
    fn with_desc_prop_adds_desc_and_desc_id_prop() {
        test_code(
            r#"<svg><g/></svg>"#,
            &core::config::Config {
                desc_prop: Some(true),
                ..Default::default()
            },
            &core::state::InternalConfig {
                ..Default::default()
            },
            r#"import * as React from "react";
const SvgComponent = ({ desc , descId  })=><svg><g/></svg>;
export default SvgComponent;
"#
        );
    }

    #[test]
    fn with_desc_prop_and_expand_props_adds_desc_desc_id_props_and_expands_prop() {
        test_code(
            r#"<svg><g/></svg>"#,
            &core::config::Config {
                expand_props: Some(core::config::ExpandProps::Bool(true)),
                desc_prop: Some(true),
                ..Default::default()
            },
            &core::state::InternalConfig {
                ..Default::default()
            },
            r#"import * as React from "react";
const SvgComponent = ({ desc , descId , ...props })=><svg><g/></svg>;
export default SvgComponent;
"#
        );
    }
}
