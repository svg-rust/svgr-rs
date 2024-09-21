use std::collections::HashMap;

use napi::{
  bindgen_prelude::{Either3, FromNapiValue, Object},
  Either,
};
use svgr_rs::{Config, ExpandProps, ExportType, Icon, JSXRuntime, JSXRuntimeImport, SvgProp};

#[derive(Clone)]
pub struct JsSvgProps(Vec<SvgProp>);

impl FromNapiValue for JsSvgProps {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<Self> {
    let js_object: Object = FromNapiValue::from_napi_value(env, napi_val)?;
    let keys = Object::keys(&js_object)?;
    let mut svg_props = Vec::with_capacity(keys.len());
    for key in keys {
      let value = js_object.get::<&str, String>(&key)?;
      if let Some(value) = value {
        svg_props.push(SvgProp { key, value });
      }
    }
    Ok(JsSvgProps(svg_props))
  }
}

#[derive(Clone)]
pub struct JsReplaceAttrValues(HashMap<String, String>);

impl FromNapiValue for JsReplaceAttrValues {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<Self> {
    let js_object: Object = FromNapiValue::from_napi_value(env, napi_val)?;
    let keys = Object::keys(&js_object)?;
    let mut replace_attr_values = HashMap::new();
    for key in keys {
      let value = js_object.get::<&str, String>(&key)?;
      if let Some(value) = value {
        replace_attr_values.insert(key, value);
      }
    }
    Ok(JsReplaceAttrValues(replace_attr_values))
  }
}

#[napi(object, object_to_js = false)]
#[derive(Clone)]
pub struct JsJSXRuntimeImport {
  pub source: String,
  pub namespace: Option<String>,
  pub default_specifier: Option<String>,
  pub specifiers: Option<Vec<String>>,
}

#[napi(object, object_to_js = false)]
#[derive(Clone)]
pub struct JsConfig {
  /// Setting this to `true` will forward ref to the root SVG tag.
  pub r#ref: Option<bool>,

  /// Add title tag via title property.
  /// If title_prop is set to true and no title is provided at render time, this will fallback to an existing title element in the svg if exists.
  pub title_prop: Option<bool>,

  /// Add desc tag via desc property.
  /// If desc_prop is set to true and no description is provided at render time, this will fallback to an existing desc element in the svg if exists.
  pub desc_prop: Option<bool>,

  /// All properties given to component will be forwarded on SVG tag.
  /// Possible values: "start", "end" or false.
  pub expand_props: Option<Either<bool, String>>,

  /// Keep `width` and `height` attributes from the root SVG tag.
  /// Removal is guaranteed if `dimensions: false`, unlike the `remove_dimensions: true` SVGO plugin option which also generates a `viewBox` from the dimensions if no `viewBox` is present.
  pub dimensions: Option<bool>,

  /// Replace SVG `width` and `height` by a custom value.
  /// If value is omitted, it uses `1em` in order to make SVG size inherits from text size.
  pub icon: Option<Either3<bool, String, f64>>,

  /// Modify all SVG nodes with uppercase and use a specific template with `react-native-svg` imports.
  /// All unsupported nodes will be removed.
  pub native: Option<bool>,

  /// Add props to the root SVG tag.
  #[napi(ts_type = "{ [key: string]: string }")]
  pub svg_props: Option<JsSvgProps>,

  /// Generates `.tsx` files with TypeScript typings.
  pub typescript: Option<bool>,

  /// Setting this to `true` will wrap the exported component in `React.memo`.
  pub memo: Option<bool>,

  /// Replace an attribute value by an other.
  /// The main usage of this option is to change an icon color to "currentColor" in order to inherit from text color.
  #[napi(ts_type = "{ [key: string]: string }")]
  pub replace_attr_values: Option<JsReplaceAttrValues>,

  /// Specify a JSX runtime to use.
  /// * "classic": adds `import * as React from 'react'` on the top of file
  /// * "automatic": do not add anything
  /// * "classic-preact": adds `import { h } from 'preact'` on the top of file
  pub jsx_runtime: Option<String>,

  /// Specify a custom JSX runtime source to use. Allows to customize the import added at the top of generated file.
  pub jsx_runtime_import: Option<JsJSXRuntimeImport>,

  /// The named export defaults to `ReactComponent`, can be customized with the `named_export` option.
  pub named_export: Option<String>,

  /// If you prefer named export in any case, you may set the `export_type` option to `named`.
  #[napi(ts_type = "'named' | 'default'")]
  pub export_type: Option<String>,
}

impl TryFrom<JsConfig> for Config {
  type Error = napi::Error;

  fn try_from(val: JsConfig) -> Result<Self, Self::Error> {
    let expand_props = match val.expand_props {
      Some(raw) => match raw {
        Either::A(b) => ExpandProps::Bool(b),
        Either::B(s) => match s.as_str() {
          "start" => ExpandProps::Start,
          "end" => ExpandProps::End,
          _ => ExpandProps::End,
        },
      },
      None => ExpandProps::End,
    };

    let icon = val.icon.map(|raw| match raw {
      Either3::A(b) => Icon::Bool(b),
      Either3::B(s) => Icon::Str(s),
      Either3::C(f) => Icon::Num(f),
    });

    let svg_props = match val.svg_props {
      Some(raw) => Some(raw.0),
      None => None,
    };

    let jsx_runtime = match val.jsx_runtime {
      Some(raw) => match raw.as_str() {
        "automatic" => JSXRuntime::Automatic,
        "classic-preact" => JSXRuntime::ClassicPreact,
        _ => JSXRuntime::Classic,
      },
      None => JSXRuntime::Classic,
    };

    let replace_attr_values = match val.replace_attr_values {
      Some(raw) => Some(raw.0),
      None => None,
    };

    let jsx_runtime_import = match val.jsx_runtime_import {
      Some(raw) => Some(JSXRuntimeImport {
        source: raw.source,
        namespace: raw.namespace,
        default_specifier: raw.default_specifier,
        specifiers: raw.specifiers,
      }),
      None => None,
    };

    let named_export = match val.named_export {
      Some(s) => s,
      None => "ReactComponent".to_string(),
    };

    let export_type = match val.export_type {
      Some(s) => match s.as_str() {
        "named" => ExportType::Named,
        _ => ExportType::Default,
      },
      None => ExportType::Default,
    };

    Ok(Self {
      r#ref: val.r#ref,
      title_prop: val.title_prop,
      desc_prop: val.desc_prop,
      expand_props,
      dimensions: val.dimensions,
      icon,
      native: val.native,
      svg_props,
      typescript: val.typescript,
      memo: val.memo,
      replace_attr_values,
      jsx_runtime: Some(jsx_runtime),
      jsx_runtime_import,
      named_export,
      export_type: Some(export_type),
    })
  }
}
