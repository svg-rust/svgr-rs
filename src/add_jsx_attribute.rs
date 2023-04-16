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

pub struct Attribute {
    pub name: String,
    pub value: Option<AttributeValue>,
    pub spread: bool,
    pub literal: bool,
    pub position: Option<AttributePosition>,
}

pub struct Visitor {
    elements: Vec<String>,
    attributes: Vec<Attribute>,
}

impl Visitor {
    pub fn new(elements: Vec<String>, attributes: Vec<Attribute>) -> Self {
        Self {
            elements,
            attributes,
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
                None => &AttributePosition::Start,
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
                        Some(JSXAttrValue::Lit(Lit::Str(Str {
                            span: DUMMY_SP,
                            value: value.to_string().into(),
                            raw: None,
                        })))
                    } else {
                        Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                            span: DUMMY_SP,
                            expr: JSXExpr::Expr(Box::new(Expr::Lit(Lit::Str(Str {
                                span: DUMMY_SP,
                                value: value.to_string().into(),
                                raw: None,
                            })))),
                        }))
                    }
                },
            }
        },
        None => None,
    }
}
