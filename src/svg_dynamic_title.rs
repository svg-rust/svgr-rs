use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        visit::VisitMut
    },
};

const ELEMENTS: [&str; 2] = ["svg", "Svg"];

pub struct Visitor {
    tag: String,
    tag_id: String,
}

impl Visitor {
    pub fn new(tag: String) -> Self {
        let tag_id = format!("{}Id", tag);

        Self {
            tag,
            tag_id,
        }
    }

    fn get_tag_expr(&self, value: JSXAttrValue) -> Expr {
        let cons = Box::new(Expr::JSXElement(Box::new(JSXElement {
            span: DUMMY_SP,
            opening: JSXOpeningElement {
                span: DUMMY_SP,
                name: JSXElementName::Ident(Ident::new(
                    self.tag.clone().into(),
                    DUMMY_SP
                )),
                attrs: vec![
                    JSXAttrOrSpread::JSXAttr(JSXAttr {
                        span: DUMMY_SP,
                        name: JSXAttrName::Ident(Ident::new(
                            "id".to_string().into(),
                            DUMMY_SP
                        )),
                        value: Some(value),
                    }),
                ],
                self_closing: false,
                type_args: None,
            },
            children: vec![
                JSXElementChild::JSXExprContainer(JSXExprContainer {
                    span: DUMMY_SP,
                    expr: JSXExpr::Expr(Box::new(Expr::Ident(Ident::new(
                        self.tag.clone().into(),
                        DUMMY_SP
                    )))),
                })
            ],
            closing: Some(JSXClosingElement {
                span: DUMMY_SP,
                name: JSXElementName::Ident(Ident::new(
                    self.tag.clone().into(),
                    DUMMY_SP
                )),
            }),
            }
        )));

        Expr::Cond(CondExpr {
            span: DUMMY_SP,
            test: Box::new(Expr::Ident(Ident::new(
                self.tag.clone().into(),
                DUMMY_SP
            ))),
            cons,
            alt: Box::new(Expr::Lit(Lit::Null(Null {
                span: DUMMY_SP
            }))),
        })
    }

    fn get_tag_element(&self) -> JSXElementChild {
        let value = JSXAttrValue::JSXExprContainer(
            JSXExprContainer {
                span: DUMMY_SP,
                expr: JSXExpr::Expr(Box::new(Expr::Ident(
                    Ident::new(
                        self.tag_id.clone().into(),
                        DUMMY_SP
                    )
                ))),
            }
        );

        let expr = self.get_tag_expr(value);
    
        JSXElementChild::JSXExprContainer(JSXExprContainer {
            span: DUMMY_SP,
            expr: JSXExpr::Expr(Box::new(expr)),
        })
    }

    fn get_tag_element_with_existing_title(&self, existing_title: &mut JSXElement) -> JSXElementChild {
        let test = Expr::Bin(BinExpr {
            span: DUMMY_SP,
            left: Box::new(Expr::Ident(Ident::new(
                self.tag.clone().into(),
                DUMMY_SP
            ))),
            op: op!("==="),
            right: Box::new(Expr::Ident(Ident::new(
                "undefined".into(),
                DUMMY_SP
            ))),
        });

        let existing_id = existing_title.opening.attrs.iter_mut().find(|attr| {
            if let JSXAttrOrSpread::JSXAttr(JSXAttr {
                name: JSXAttrName::Ident(ident),
                ..
            }) = attr {
                if ident.sym.to_string() == "id" {
                    return true
                }
            }
            false
        });

        let id_attr_value = if let Some(JSXAttrOrSpread::JSXAttr(attr)) = existing_id {
            let jsx_attr_value = match &attr.value {
                Some(JSXAttrValue::Lit(Lit::Str(Str {
                    value,
                    ..
                }))) => {
                    JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        span: DUMMY_SP,
                        expr: JSXExpr::Expr(Box::new(Expr::Bin(BinExpr {
                            span: DUMMY_SP,
                            left: Box::new(Expr::Ident(Ident::new(
                                self.tag_id.clone().into(),
                                DUMMY_SP
                            ))),
                            op: op!("||"),
                            right: Box::new(Expr::Lit(Lit::Str(Str {
                                span: DUMMY_SP,
                                value: value.clone(),
                                raw: None,
                            }))),
                        }))),
                    })
                },
                _ => JSXAttrValue::JSXExprContainer(JSXExprContainer {
                    span: DUMMY_SP,
                    expr: JSXExpr::Expr(Box::new(Expr::Ident(Ident::new(
                        self.tag_id.clone().into(),
                        DUMMY_SP
                    )))),
                }),
            };

            attr.value = Some(jsx_attr_value.clone());

            jsx_attr_value
        } else {
            let jsx_attr_value = JSXAttrValue::JSXExprContainer(
                JSXExprContainer {
                    span: DUMMY_SP,
                    expr: JSXExpr::Expr(Box::new(Expr::Ident(Ident::new(
                        self.tag_id.clone().into(),
                        DUMMY_SP
                    )))),
                }
            );

            let id_attr = JSXAttrOrSpread::JSXAttr(JSXAttr {
                span: DUMMY_SP,
                name: JSXAttrName::Ident(Ident::new(
                    "id".into(),
                    DUMMY_SP,
                )),
                value: Some(jsx_attr_value.clone()),
            });
            existing_title.opening.attrs.push(id_attr);

            jsx_attr_value
        };

        if existing_title.children.len() == 0 {
            return JSXElementChild::JSXExprContainer(JSXExprContainer {
                span: DUMMY_SP,
                expr: JSXExpr::Expr(Box::new(self.get_tag_expr(id_attr_value))),
            })
        }

        JSXElementChild::JSXExprContainer(JSXExprContainer {
            span: DUMMY_SP,
            expr: JSXExpr::Expr(Box::new(Expr::Cond(CondExpr {
                span: DUMMY_SP,
                test: Box::new(test),
                cons: Box::new(Expr::JSXElement(Box::new(existing_title.clone()))),
                alt: Box::new(self.get_tag_expr(id_attr_value)),
            })))
        })
    }
}

impl VisitMut for Visitor {
    fn visit_mut_jsx_element(&mut self, n: &mut JSXElement) {
        if let JSXElementName::Ident(ident) = &n.opening.name {
            let name = ident.sym.to_string();
            if !ELEMENTS.iter().any(|e| *e == name) {
                return
            }

            let has_tag = n.children.clone().iter_mut().enumerate().any(|(i, c)| {
                if let JSXElementChild::JSXElement(e) = c {
                    if let JSXElementName::Ident(ident) = &e.opening.name {
                        if ident.sym.to_string() == self.tag {
                            let tag_element = self.get_tag_element_with_existing_title(e);
                            n.children[i] = tag_element;
                            return true
                        }
                    }
                }
                false
            });

            if !has_tag {
                n.children.insert(0, self.get_tag_element());
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

    fn code_test(input: &str, tag: String, expected: &str) {
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

        let module = module.fold_with(&mut as_folder(Visitor::new(tag)));

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
    fn title_plugin_should_add_title_attribute_if_not_present() {
        code_test(
            r#"<svg></svg>;"#,
            "title".to_string(),
            r#"<svg>{title ? <title id={titleId}>{title}</title> : null}</svg>;"#,
        );
    }

    #[test]
    fn title_plugin_should_add_title_element_and_fallback_to_existing_title() {
        code_test(
            r#"<svg><title>Hello</title></svg>;"#,
            "title".to_string(),
            r#"<svg>{title === undefined ? <title id={titleId}>Hello</title> : title ? <title id={titleId}>{title}</title> : null}</svg>;"#,
        );

        code_test(
            r#"<svg><title>{"Hello"}</title></svg>;"#,
            "title".to_string(),
            r#"<svg>{title === undefined ? <title id={titleId}>{"Hello"}</title> : title ? <title id={titleId}>{title}</title> : null}</svg>;"#,
        );
    }

    #[test]
    fn title_plugin_should_preserve_any_existing_title_attributes() {
        code_test(
            r#"<svg><title id="a">Hello</title></svg>;"#,
            "title".to_string(),
            r#"<svg>{title === undefined ? <title id={titleId || "a"}>Hello</title> : title ? <title id={titleId || "a"}>{title}</title> : null}</svg>;"#,
        );
    }

    #[test]
    fn title_plugin_should_support_empty_title() {
        code_test(
            r#"<svg><title></title></svg>;"#,
            "title".to_string(),
            r#"<svg>{title ? <title id={titleId}>{title}</title> : null}</svg>;"#,
        );
    }

    #[test]
    fn title_plugin_should_support_self_closing_title() {
        code_test(
            r#"<svg><title/></svg>;"#,
            "title".to_string(),
            r#"<svg>{title ? <title id={titleId}>{title}</title> : null}</svg>;"#,
        );
    }

    #[test]
    fn title_plugin_should_work_if_an_attribute_is_already_present() {
        code_test(
            r#"<svg><foo/></svg>;"#,
            "title".to_string(),
            r#"<svg>{title ? <title id={titleId}>{title}</title> : null}<foo/></svg>;"#,
        );
    }
}
