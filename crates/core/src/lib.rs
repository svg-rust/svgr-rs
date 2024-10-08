#![feature(path_file_prefix)]
#![deny(clippy::all)]

use std::rc::Rc;

use swc_core::{
  common::{comments::SingleThreadedComments, FileName, SourceMap},
  ecma::{
    codegen::{text_writer::JsWriter, Emitter},
    visit::{as_folder, FoldWith},
  },
};
use swc_xml::parser::parse_file_as_document;

mod add_jsx_attribute;
mod core;
mod error;
mod hast_to_swc_ast;
mod remove_jsx_attribute;
mod replace_jsx_attribute;
mod svg_dynamic_title;
mod svg_em_dimensions;
mod transform_react_native_svg;
mod transform_svg_component;

pub use error::SvgrError;

pub use self::core::config::{
  Config, ExpandProps, ExportType, Icon, JSXRuntime, JSXRuntimeImport, SvgProp,
};
pub use self::core::state::{Caller, Config as State};

/// Transform SVG into React components.
///
/// It takes three arguments:
///
/// * source: the SVG source code to transform
/// * options: the options used to transform the SVG
/// * state: a state linked to the transformation
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use svgr_rs::transform;
///
/// let result = transform(
///   r#"<svg></svg>"#.to_string(),
///   Default::default(),
///   Default::default(),
/// );
/// ```
pub fn transform(code: String, config: Config, state: State) -> Result<String, SvgrError> {
  let state = core::state::expand_state(&state);

  let cm = Rc::<SourceMap>::default();
  let fm = cm.new_source_file(FileName::Anon.into(), code);

  let mut errors = vec![];
  let document = parse_file_as_document(fm.as_ref(), Default::default(), &mut errors)
    .map_err(|e| SvgrError::Parse(e.message().to_string()))?;

  let jsx_element = hast_to_swc_ast::to_swc_ast(document);
  if jsx_element.is_none() {
    return Err(SvgrError::InvalidSvg);
  }
  let jsx_element = jsx_element.unwrap();

  let m = transform_svg_component::transform(jsx_element, &config, &state)?;

  let m = m.fold_with(&mut as_folder(remove_jsx_attribute::Visitor::new(&config)));
  let m = m.fold_with(&mut as_folder(add_jsx_attribute::Visitor::new(&config)));

  let icon = match config.icon {
    Some(core::config::Icon::Bool(b)) => b,
    None => false,
    _ => true,
  };
  let m = if icon && config.dimensions {
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

  let m = if config.title_prop {
    m.fold_with(&mut as_folder(svg_dynamic_title::Visitor::new(
      "title".to_string(),
    )))
  } else {
    m
  };

  let m = if config.desc_prop {
    m.fold_with(&mut as_folder(svg_dynamic_title::Visitor::new(
      "desc".to_string(),
    )))
  } else {
    m
  };

  let m = if config.native {
    let comments = SingleThreadedComments::default();
    m.fold_with(&mut as_folder(transform_react_native_svg::Visitor::new(
      &comments,
    )))
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
