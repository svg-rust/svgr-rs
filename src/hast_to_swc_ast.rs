use swc::atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecmascript::ast::*;
use swc_xml::{visit::{Visit, VisitWith}};
use regex::{Regex, Captures};

mod string_to_object_style;

fn kebab_case(str: &str) -> String {
    let kebab_regex = Regex::new(r"[A-Z\u00C0-\u00D6\u00D8-\u00DE]").unwrap();
    kebab_regex.replace_all(str, |caps: &Captures| format!("-{}", &caps[0].to_lowercase())).into()
}

fn convert_aria_attribute(kebab_key: &str) -> String {
    let parts: Vec<&str> = kebab_key.split('-').collect();
    let aria = parts[0];
    let lowercase_parts: String = parts[1..].join("").to_lowercase();
    format!("{}-{}", aria, lowercase_parts)
}

fn get_key(attr_name: &str, tag_name: &str) -> Ident {
    let lower_case_name = attr_name.to_lowercase();
    let rc_key = {
        match tag_name {
            "input" => {
                match lower_case_name.as_str() {
                    "checked" => Some("defaultChecked"),
                    "value" => Some("defaultValue"),
                    "maxlength" => Some("maxLength"),
                    _ => None,
                }
            },
            "form" => {
                match lower_case_name.as_str() {
                    "enctype" => Some("encType"),
                    _ => None,
                }
            },
            _ => None,
        }
    };

    if let Some(k) = rc_key {
        return Ident {
            span: DUMMY_SP,
            sym: k.into(),
            optional: false,
        }
    }

    let kebab_key = kebab_case(&attr_name);

    if kebab_key.starts_with("aria-") {
        return Ident {
            span: DUMMY_SP,
            sym: convert_aria_attribute(attr_name).into(),
            optional: false,
        }
    }

    if kebab_key.starts_with("data-") {
        return Ident {
            span: DUMMY_SP,
            sym: attr_name.clone().into(),
            optional: false,
        }
    }

    Ident {
        span: DUMMY_SP,
        sym: attr_name.clone().into(),
        optional: false,
    }
}

fn get_value(attr_name: &str, value: &JsWord) -> JSXAttrValue {
    if attr_name == "style" {
        let style = string_to_object_style::string_to_object_style(value);

        return JSXAttrValue::JSXExprContainer(JSXExprContainer {
            span: DUMMY_SP,
            expr: JSXExpr::Expr(Box::new(style)),
        })
    }

    return JSXAttrValue::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: value.clone(),
        raw: None
    }))
}

fn all(children: &Vec<swc_xml::ast::Child>) -> Vec<JSXElementChild> {
    children.into_iter()
        .map(|n| {
            match n {
                swc_xml::ast::Child::Element(e) => Some(JSXElementChild::JSXElement(Box::new(element(&e)))),
                swc_xml::ast::Child::Text(t) => Some(JSXElementChild::JSXText(text(&t))),
                _ => None,
            }
        })
        .filter(|n| n.is_some())
        .map(|n| n.unwrap())
        .collect()
}

fn comment(n: &swc_xml::ast::Comment) -> JSXText {
    todo!()
}

fn text(n: &swc_xml::ast::Text) -> JSXText {
    todo!()
}

fn element(n: &swc_xml::ast::Element) -> JSXElement {
    let attrs = n.attributes.iter().map(
        |attr| {
            let value = match attr.value.clone() {
                Some(v) => Some(get_value(&attr.name, &v)),
                None => None,
            };

            JSXAttrOrSpread::JSXAttr(JSXAttr {
                span: DUMMY_SP,
                name: JSXAttrName::Ident(get_key(&attr.name, &n.tag_name)),
                value,
            })
        }
    ).collect::<Vec<JSXAttrOrSpread>>();

    let name = JSXElementName::Ident(Ident::new(n.tag_name.clone(), DUMMY_SP));
    let children = all(&n.children);

    let opening = JSXOpeningElement {
        span: DUMMY_SP,
        name: name.clone(),
        attrs,
        self_closing: children.len() == 0,
        type_args: None,
    };

    let closing = if children.len() > 0 {
        Some(JSXClosingElement {
            span: DUMMY_SP,
            name,
        })
    } else {
        None
    };

    JSXElement {
        span: DUMMY_SP,
        opening,
        children,
        closing,
    }
}

pub struct HastVisitor {
    jsx: Option::<JSXElement>,
}

impl HastVisitor {
    pub fn get_jsx(&self) -> Option::<JSXElement> {
        self.jsx.clone()
    }
}

impl Visit for HastVisitor {
    fn visit_element(&mut self, n: &swc_xml::ast::Element) {
        self.jsx = Some(element(n));
    }
}

pub fn to_swc_ast(hast: swc_xml::ast::Document) -> Option<JSXElement> {
    let mut v = HastVisitor {
        jsx: None,
    };
    hast.visit_with(&mut v);
    v.get_jsx()
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, borrow::Borrow};
    use swc_common::{SourceMap, FileName};
    use swc_ecmascript::{codegen::{text_writer::JsWriter, Emitter, Config}};
    use swc_xml::parser::{parse_file_as_document, parser};

    use super::*;

    fn transform(code: &str) -> String {
        let cm = Arc::<SourceMap>::default();

        let fm = cm.new_source_file(FileName::Anon, code.to_string());

        let mut errors = vec![];
        let document = parse_file_as_document(
            fm.borrow(),
            parser::ParserConfig {
                ..Default::default()
            },
            &mut errors
        ).unwrap();

        match to_swc_ast(document) {
            Some(jsx) => {
                let code = {
                    let mut buf = vec![];
            
                    {
                        let mut emitter = Emitter {
                            cfg: Config {
                                minify: true,
                                ..Default::default()
                            },
                            cm: cm.clone(),
                            comments: None,
                            wr: JsWriter::new(cm, "", &mut buf, None),
                        };
            
                        emitter.emit_module_item(&ModuleItem::Stmt(
                            Stmt::Expr(ExprStmt {
                                span: DUMMY_SP,
                                expr: Box::new(Expr::JSXElement(Box::new(jsx))),
                            })
                        )).unwrap();
                    }
            
                    String::from_utf8_lossy(&buf).to_string()
                };
                
                code
            },
            None => panic!("No JSX element found"),
        }
    }

    #[test]
    fn transforms_aria_x() {
        let svg = r#"<svg aria-hidden="true"></svg>"#;
        let jsx = transform(svg);
        assert_eq!(jsx, r#"<svg aria-hidden="true"/>;"#)
    }

    #[test]
    fn transforms_style() {
        let svg = r#"<svg><path style="--index: 1; font-size: 24px;"></path><path style="--index: 2"></path></svg>"#;
        let jsx = transform(svg);
        assert_eq!(jsx, r#"<svg><path style={{"--index":1,fontSize:24}}/><path style={{"--index":2}}/></svg>;"#)
    }
}
