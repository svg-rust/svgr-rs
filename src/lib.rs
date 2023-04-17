#![feature(path_file_prefix)]

#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::{sync::Arc, borrow::Borrow};
use swc_xml::{parser::{parse_file_as_document, parser}};
use swc_core::{
    common::{SourceMap, FileName},
    ecma::{
        codegen::{text_writer::JsWriter, Emitter, Config},
        visit::{FoldWith, as_folder},
    },
    node::get_deserialized,
};
use napi::bindgen_prelude::*;

mod hast_to_swc_ast;
mod core;

mod add_jsx_attribute;
mod svg_em_dimensions;
mod transform_svg_component;

#[napi]
pub async fn transform(code: String, config: Buffer, state: Option<core::state::Config>) -> Result<String> {
    let config: core::config::Config = get_deserialized(&config)?;
    let state = core::state::expand_state(state.as_ref());

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

    let jsx_element = hast_to_swc_ast::to_swc_ast(document);

    let jsx_element = match jsx_element {
        Some(jsx_element) => jsx_element,
        None => panic!("This is invalid SVG")
    };

    let m =  transform_svg_component::transform(jsx_element, &config, &state);

    let m = m.fold_with(&mut as_folder(add_jsx_attribute::Visitor::new(&config)));
    let m = m.fold_with(&mut as_folder(svg_em_dimensions::Visitor::new(&config)));

    let mut buf = vec![];

    let mut emitter = Emitter {
        cfg: Config {
            ..Default::default()
        },
        cm: cm.clone(),
        comments: None,
        wr: JsWriter::new(cm, "\n", &mut buf, None),
    };
    emitter.emit_module(&m).unwrap();

    Ok(String::from_utf8_lossy(&buf).to_string())
}
