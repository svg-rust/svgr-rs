use swc_common::DUMMY_SP;
use swc_ecmascript::ast::*;
use swc_xml::{visit::{Visit, VisitWith}, ast::{Document}};

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
        println!("{:?}", n.attributes);

        let opening = JSXOpeningElement {
            span: DUMMY_SP,
            name: JSXElementName::Ident(Ident::new(n.tag_name.clone(), DUMMY_SP)),
            attrs: vec![],
            self_closing: true,
            type_args: None,
        };

        let e = JSXElement {
            span: DUMMY_SP,
            opening,
            children: vec![],
            closing: None,
        };

        self.jsx = Some(e);
    }
}

pub fn to_swc_ast(hast: Document) -> Option<JSXElement> {
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
            parser::ParserConfig{
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
                                ..Default::default()
                            },
                            cm: cm.clone(),
                            comments: None,
                            wr: JsWriter::new(cm, "\n", &mut buf, None),
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
        let code = "<svg aria-hidden=\"true\"></svg>";
        let result = transform(code);
        println!("{}", result);
        assert!(result == "<svg aria-hidden=\"true\" />;")
    }
}
