use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        visit::VisitMut
    },
};

const ELEMENTS: [&str; 2] = ["svg", "Svg"];

enum StrOrNum {
    Str(String),
    Num(f64),
}

pub struct Visitor {
    height: Option<StrOrNum>,
    width: Option<StrOrNum>,
}

impl Visitor {
    pub fn new() -> Self {
        Self {
            height: Some(StrOrNum::Str("1em".to_string())),
            width: Some(StrOrNum::Str("1em".to_string())),
        }
    }
}

impl VisitMut for Visitor {
    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        let is_svg = ELEMENTS.iter().any(|element| {
            if let JSXElementName::Ident(e) = n.name.clone() {
                return e.sym.to_string() == *element
            }
            false
        });

        if !is_svg {
            return;
        }

        let mut required_attrs = vec!["width", "height"];

        n.attrs.iter_mut().for_each(|attr| {
            if let JSXAttrOrSpread::JSXAttr(jsx_attr) = attr {
                if let JSXAttrName::Ident(ident) = jsx_attr.name.clone() {
                    required_attrs.clone().iter().enumerate().for_each(|(index, attr)| {
                        if ident.sym.to_string() == *attr {
                            match *attr {
                                "height" => {
                                    jsx_attr.value.replace(get_value(self.height.as_ref()));
                                },
                                "width" => {
                                    jsx_attr.value.replace(get_value(self.width.as_ref()));
                                },
                                _ => {}
                            }
                            required_attrs.remove(index);
                        }
                    });
                }
            }
        });

        required_attrs.iter().for_each(|attr| {
            n.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                span: DUMMY_SP,
                name: JSXAttrName::Ident(Ident::new((*attr).into(), DUMMY_SP)),
                value: Some(get_value(match *attr {
                    "height" => self.height.as_ref(),
                    "width" => self.width.as_ref(),
                    _ => None,
                })),
            }));
        });
    }
}

fn get_value(raw: Option<&StrOrNum>) -> JSXAttrValue {
    match raw {
        None => {
            JSXAttrValue::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: "1em".into(),
                raw: None,
            }))
        },
        Some(str_or_num) => {
            match str_or_num {
                StrOrNum::Str(str) => {
                    JSXAttrValue::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: str.clone().into(),
                        raw: None,
                    }))
                },
                StrOrNum::Num(num) => {
                    JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        expr: JSXExpr::Expr(Box::new(Expr::Lit(Lit::Num(Number {
                            span: DUMMY_SP,
                            value: num.clone(),
                            raw: None,
                        })))),
                        span: DUMMY_SP,
                    })
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use swc_core::{
        common::{SourceMap, FileName},
        ecma::{
            ast::*,
            parser::{lexer::Lexer, Parser, StringInput, Syntax, EsConfig},
            codegen::{text_writer::JsWriter, Emitter, Config},
            visit::{FoldWith, as_folder}
        },
    };

    use super::*;

    struct Options {
        height: Option<StrOrNum>,
        width: Option<StrOrNum>,
    }

    fn code_test(input: &str, opts: Options, expected: &str) {
        let cm = Arc::<SourceMap>::default();
        let fm = cm.new_source_file(FileName::Anon, input.to_string());

        let lexer = Lexer::new(
            Syntax::Es(EsConfig {
                decorators: true,
                jsx: true,
                ..Default::default()
            }),
            EsVersion::EsNext,
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_module().unwrap();

        let module = module.fold_with(&mut as_folder(Visitor {
            height: opts.height,
            width: opts.width,
        }));

        let mut buf = vec![];
        let mut emitter = Emitter {
            cfg: Config {
                ..Default::default()
            },
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm, "", &mut buf, None),
        };
        emitter.emit_module(&module).unwrap();
        let result = String::from_utf8_lossy(&buf).to_string();

        assert_eq!(result, expected)
    }

    #[test]
    fn replaces_width_or_height_attributes() {
        code_test(
            r#"<svg foo="bar" width={100} height={200}/>;"#,
            Options {
                height: None,
                width: None,
            },
            r#"<svg foo="bar" width="1em" height="1em"/>;"#,
        );
    }

    #[test]
    fn adds_em_if_they_are_not_present() {
        code_test(
            r#"<svg foo="bar"/>;"#,
            Options {
                height: None,
                width: None,
            },
            r#"<svg foo="bar" width="1em" height="1em"/>;"#,
        );
    }

    #[test]
    fn accepts_numeric_values() {
        code_test(
            r#"<svg foo="bar"/>;"#,
            Options {
                height: Some(StrOrNum::Num(24.0)),
                width: Some(StrOrNum::Num(24.0)),
            },
            r#"<svg foo="bar" width={24} height={24}/>;"#,
        );
    }

    #[test]
    fn accepts_string_values() {
        code_test(
            r#"<svg foo="bar"/>;"#,
            Options {
                height: Some(StrOrNum::Str("2em".to_string())),
                width: Some(StrOrNum::Str("2em".to_string())),
            },
            r#"<svg foo="bar" width="2em" height="2em"/>;"#,
        );
    }
}
