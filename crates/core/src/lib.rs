#![feature(path_file_prefix)]

#![deny(clippy::all)]

#[cfg(feature = "node")]
#[macro_use]
extern crate napi_derive;

use std::{sync::Arc, borrow::Borrow};
use swc_xml::{parser::{parse_file_as_document, parser}};
use swc_core::{
    common::{SourceMap, FileName, comments::SingleThreadedComments},
    ecma::{
        codegen::{text_writer::JsWriter, Emitter},
        visit::{FoldWith, as_folder},
    },
};

mod hast_to_swc_ast;
mod core;
mod transform_svg_component;
mod add_jsx_attribute;
mod remove_jsx_attribute;
mod replace_jsx_attribute;
mod svg_dynamic_title;
mod svg_em_dimensions;
mod transform_react_native_svg;

pub use self::core::config::Config as Config;
pub use self::core::state::Config as State;

pub fn transform(code: String, config: Config, state: State) -> Result<String, String> {
    let state = core::state::expand_state(&state);

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
    if jsx_element.is_none() {
        return Err("This is invalid SVG".to_string());
    }
    let jsx_element = jsx_element.unwrap();

    let m = transform_svg_component::transform(jsx_element, &config, &state);
    if m.is_err() {
        return Err(m.unwrap_err());
    }
    let m = m.unwrap();

    let m = m.fold_with(&mut as_folder(remove_jsx_attribute::Visitor::new(&config)));
    let m = m.fold_with(&mut as_folder(add_jsx_attribute::Visitor::new(&config)));

    let icon = match config.icon {
        Some(core::config::Icon::Bool(b)) => b,
        None => false,
        _ => true
    };
    let dimensions = config.dimensions.unwrap_or(true);
    let m = if icon && dimensions {
        m.fold_with(&mut as_folder(svg_em_dimensions::Visitor::new(&config)))
    } else {
        m
    };

    let replace_attr_values = config.replace_attr_values.is_some();
    let m = if replace_attr_values {
        m.fold_with(&mut as_folder(replace_jsx_attribute::Visitor::new(&config)))
    } else {
        m
    };

    let title_prop = config.title_prop.unwrap_or(false);
    let m = if title_prop {
        m.fold_with(&mut as_folder(svg_dynamic_title::Visitor::new("title".to_string())))
    } else {
        m
    };

    let desc_prop = config.desc_prop.unwrap_or(false);
    let m = if desc_prop {
        m.fold_with(&mut as_folder(svg_dynamic_title::Visitor::new("desc".to_string())))
    } else {
        m
    };

    let native = config.native.unwrap_or(false);
    let m = if native {
        let comments = SingleThreadedComments::default();
        m.fold_with(&mut as_folder(transform_react_native_svg::Visitor::new(&comments)))
    } else {
        m
    };

    let mut buf = vec![];

    let mut emitter = Emitter {
        cfg: Default::default(),
        cm: cm.clone(),
        comments: None,
        wr: JsWriter::new(cm, "\n", &mut buf, None),
    };
    emitter.emit_module(&m).unwrap();

    Ok(String::from_utf8_lossy(&buf).to_string())
}
