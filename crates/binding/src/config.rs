use napi::{bindgen_prelude::Either3, Either, JsObject};
use svgr_rs::{Config, ExpandProps, Icon, JSXRuntime, JSXRuntimeImport};

#[napi(object, object_to_js = false)]
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
  pub svg_props: Option<JsObject>,

  /// Generates `.tsx` files with TypeScript typings.
  pub typescript: Option<bool>,

  /// Setting this to `true` will wrap the exported component in `React.memo`.
  pub memo: Option<bool>,

  /// Replace an attribute value by an other.
  /// The main usage of this option is to change an icon color to "currentColor" in order to inherit from text color.
  pub replace_attr_values: Option<JsObject>,

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

impl From<JsConfig> for Config {
    fn from(val: JsConfig) -> Self {
      let expand_props = match val.expand_props {
        Some(e) => match e {
            Either::A(b) => ExpandProps::Bool(b),
            Either::B(s) => match s.as_str() {
              "start" => ExpandProps::Start,
              "end" => ExpandProps::End,
              _ => ExpandProps::End
            },
        },
        None => ExpandProps::End,
    };

    let icon = match val.icon {
        Some(i) => Some(match i {
            Either3::A(b) => Icon::Bool(b),
            Either3::B(s) => Icon::Str(s),
            Either3::C(f) => Icon::Num(f),
        }),
        None => None,
    };

        Self {
            _ref: val.r#ref,
            title_prop: val.title_prop,
            desc_prop: val.desc_prop,
            expand_props,
            dimensions: val.dimensions,
            icon,
            native: val.native,
            svg_props: val.svg_props,
            typescript: val.typescript,
            memo: val.memo,
            replace_attr_values: val.replace_attr_values,
            jsx_runtime: val.jsx_runtime,
            jsx_runtime_import: val.jsx_runtime_import,
            named_export: val.named_export,
            export_type: val.export_type,
        }
    }
}

#[napi(object, object_to_js = false)]
pub struct JsJSXRuntimeImport {
    pub source: String,
  pub namespace: Option<String>,
  pub default_specifier: Option<String>,
  pub specifiers: Option<Vec<String>>,
}
