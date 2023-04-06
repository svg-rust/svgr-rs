use swc_common::DUMMY_SP;
use swc_ecmascript::ast::*;
use swc_xml::visit::Visit;

pub struct SvgToReactAst<'a> {
    pub body: &'a mut Vec<ModuleItem>,
}

impl Visit for SvgToReactAst<'_> {
    fn visit_element(&mut self, n: &swc_xml::ast::Element) {
        let opening = JSXOpeningElement {
            span: DUMMY_SP,
            name: JSXElementName::Ident(Ident::new("a".into(), DUMMY_SP)),
            attrs: vec![],
            self_closing: true,
            type_args: None,
        };

        let element = JSXElement {
            span: DUMMY_SP,
            opening,
            children: vec![],
            closing: None,
        };

        let stmt = ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(Expr::JSXElement(Box::new(element)))
        };

        self.body.push(ModuleItem::Stmt(Stmt::Expr(stmt)));
    }
}