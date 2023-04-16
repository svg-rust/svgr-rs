use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::*,
        visit::VisitMut
    },
};

pub enum AttributeValue {
    Bool(bool),
    Num(f64),
    Str(String),
}

pub enum AttributePosition {
    Start,
    End,
}

#[derive(Default)]
pub struct Attribute {
    pub name: String,
    pub value: Option<AttributeValue>,
    pub spread: bool,
    pub literal: bool,
    pub position: Option<AttributePosition>,
}

pub struct Options {
    elements: Vec<String>,
    attributes: Vec<Attribute>,
}

pub struct Visitor {
    elements: Vec<String>,
    attributes: Vec<Attribute>,
}

impl Visitor {
    pub fn new(opts: Options) -> Self {
        Self {
            elements: opts.elements,
            attributes: opts.attributes,
        }
    }
}

impl VisitMut for Visitor {
    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        if let JSXElementName::Ident(ident) = n.name.clone() {
            if !self.elements.contains(&ident.sym.to_string()) {
                return;
            }
        } else {
            return;
        }     
        
        for attribute in self.attributes.iter() {
            let Attribute {
                name,
                value,
                spread,
                literal,
                position,
            } = attribute;

            let position = match position {
                Some(position) => position,
                None => &AttributePosition::End,
            };

            let new_attr = get_attr(*spread, &name, value.as_ref(), *literal);

            let is_equal_attr = |attr: JSXAttrOrSpread| -> bool {
                if *spread {
                    if let JSXAttrOrSpread::SpreadElement(spread) = attr {
                        if let Expr::Ident(ident) = spread.expr.as_ref() {
                            return ident.sym.to_string() == *name
                        }
                    }
                    false
                } else {
                    if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                        if let JSXAttrName::Ident(ident) = attr.name.clone() {
                            return ident.sym.to_string() == *name
                        }
                    }
                    false
                }
            };
        
            let replaced = n.attrs.clone().iter().enumerate().any(|(index, attr)| {
                if !is_equal_attr(attr.clone()) {
                    return false
                }
                n.attrs[index] = new_attr.clone();
                true
            });
 
            if !replaced {
                match position {
                    AttributePosition::Start => {
                        n.attrs.insert(0, new_attr);
                    },
                    AttributePosition::End => {
                        n.attrs.push(new_attr);
                    },
                }
            }
        }        
    }
}

fn get_attr(spread: bool, name: &str, value: Option<&AttributeValue>, literal: bool) -> JSXAttrOrSpread {
    if spread {
        JSXAttrOrSpread::SpreadElement(SpreadElement {
            dot3_token: DUMMY_SP,
            expr: Box::new(
                Expr::Ident(Ident {
                sym: name.to_string().into(),
                span: DUMMY_SP,
                optional: false,
            })),
        })
    } else {
        JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: DUMMY_SP,
            name: JSXAttrName::Ident(Ident {
                sym: name.to_string().into(),
                span: DUMMY_SP,
                optional: false,
            }),
            value: get_attr_value(literal, value),
        })
    }
}

fn get_attr_value(literal: bool, attr_value: Option<&AttributeValue>) -> Option<JSXAttrValue> {
    match attr_value {
        Some(value) => {
            match value {
                AttributeValue::Bool(value) => {
                    Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        expr: JSXExpr::Expr(Box::new(Expr::Lit(Lit::Bool(Bool {
                            span: DUMMY_SP,
                            value: value.clone(),
                        })))),
                        span: DUMMY_SP,
                    }))
                },
                AttributeValue::Num(value) => {
                    Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        expr: JSXExpr::Expr(Box::new(Expr::Lit(Lit::Num(Number {
                            span: DUMMY_SP,
                            value: value.clone(),
                            raw: None,
                        })))),
                        span: DUMMY_SP,
                    }))
                },
                AttributeValue::Str(value) => {
                    if literal {
                        Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            span: DUMMY_SP,
                            expr: JSXExpr::Expr(Box::new(Expr::Ident(Ident {
                                sym: value.to_string().into(),
                                span: DUMMY_SP,
                                optional: false,
                            }))),
                        }))
                    } else {
                        Some(JSXAttrValue::Lit(Lit::Str(Str {
                            span: DUMMY_SP,
                            value: value.to_string().into(),
                            raw: None,
                        })))
                    }
                },
            }
        },
        None => None,
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

        let module = module.fold_with(&mut as_folder(Visitor::new(opts)));

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
    fn should_add_simple_attribute() {
        code_test(
            r#"<div/>;"#,
            Options {
                elements: vec!["div".to_string()],
                attributes: vec![
                    Attribute {
                        name: "disabled".to_string(),
                        ..Attribute::default()
                    }
                ],
            },
            r#"<div disabled/>;"#,
        );
    }

    #[test]
    fn should_add_attribute_with_value() {
        code_test(
            r#"<div/>;"#,
            Options {
                elements: vec!["div".to_string()],
                attributes: vec![
                    Attribute {
                        name: "disabled".to_string(),
                        value: Some(AttributeValue::Bool(true)),
                        ..Attribute::default()
                    }
                ],
            },
            r#"<div disabled={true}/>;"#,
        );

        code_test(
            r#"<div/>;"#,
            Options {
                elements: vec!["div".to_string()],
                attributes: vec![
                    Attribute {
                        name: "disabled".to_string(),
                        value: Some(AttributeValue::Str("true".to_string())),
                        ..Attribute::default()
                    }
                ],
            },
            r#"<div disabled="true"/>;"#,
        );

        code_test(
            r#"<div/>;"#,
            Options {
                elements: vec!["div".to_string()],
                attributes: vec![
                    Attribute {
                        name: "disabled".to_string(),
                        value: Some(AttributeValue::Num(200.0)),
                        ..Attribute::default()
                    }
                ],
            },
            r#"<div disabled={200}/>;"#,
        );
    }

    #[test]
    fn should_add_literal_attribute() {
        code_test(
            r#"<div/>;"#,
            Options {
                elements: vec!["div".to_string()],
                attributes: vec![
                    Attribute {
                        name: "ref".to_string(),
                        value: Some(AttributeValue::Str("ref".to_string())),
                        literal: true,
                        ..Attribute::default()
                    }
                ],
            },
            r#"<div ref={ref}/>;"#,
        );

        code_test(
            r#"<div/>;"#,
            Options {
                elements: vec!["div".to_string()],
                attributes: vec![
                    Attribute {
                        name: "ref".to_string(),
                        value: Some(AttributeValue::Str("ref ? ref : null".to_string())),
                        literal: true,
                        ..Attribute::default()
                    }
                ],
            },
            r#"<div ref={ref ? ref : null}/>;"#,
        );
    }

    #[test]
    fn should_add_spread_attribute() {
        code_test(
            r#"<div foo><span/></div>;"#,
            Options {
                elements: vec!["div".to_string()],
                attributes: vec![
                    Attribute {
                        name: "props".to_string(),
                        position: Some(AttributePosition::Start),
                        spread: true,
                        ..Attribute::default()
                    }
                ],
            },
            r#"<div {...props} foo><span/></div>;"#,
        );

        code_test(
            r#"<div><span foo="bar"/></div>;"#,
            Options {
                elements: vec!["span".to_string()],
                attributes: vec![
                    Attribute {
                        name: "props".to_string(),
                        position: Some(AttributePosition::End),
                        spread: true,
                        ..Attribute::default()
                    }
                ],
            },
            r#"<div><span foo="bar" {...props}/></div>;"#,
        );
    }

    #[test]
    fn should_replace_attribute() {
        code_test(
            r#"<div disabled/>;"#,
            Options {
                elements: vec!["div".to_string()],
                attributes: vec![
                    Attribute {
                        name: "disabled".to_string(),
                        value: Some(AttributeValue::Bool(false)),
                        ..Attribute::default()
                    }
                ],
            },
            r#"<div disabled={false}/>;"#,
        );
    }
}
