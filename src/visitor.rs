use swc_common::DUMMY_SP;
use swc_ecmascript::ast::*;
use swc_xml::visit::Visit;

pub struct SvgToReactAst {
    jsx: Option::<JSXElement>,
}

impl SvgToReactAst {
    pub fn new() -> Self {
        SvgToReactAst {
            jsx: None,
        }
    }

    pub fn get_jsx(&self) -> Option::<JSXElement> {
        self.jsx.clone()
    }
}

impl Visit for SvgToReactAst {
    fn visit_element(&mut self, n: &swc_xml::ast::Element) {
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