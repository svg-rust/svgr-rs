use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;

#[derive(Debug, Clone)]
pub enum Icon {
  Bool(bool),
  Str(String),
  Num(f64),
}

impl Default for Icon {
  fn default() -> Self {
    Icon::Bool(false)
  }
}

#[derive(Debug, Clone, Default)]
pub enum ExpandProps {
  Bool(bool),
  Start,
  #[default]
  End,
}

#[derive(Debug, Clone)]
pub enum JSXRuntime {
  Classic,
  ClassicPreact,
  Automatic,
}

#[derive(Debug, Clone, Default)]
pub struct JSXRuntimeImport {
  pub source: String,
  pub namespace: Option<String>,
  pub default_specifier: Option<String>,
  pub specifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum ExportType {
  Named,
  Default,
}

/// The options used to transform the SVG.
#[derive(Debug, Clone)]
pub struct Config {
  /// Setting this to `true` will forward ref to the root SVG tag.
  pub _ref: Option<bool>,

  /// Add title tag via title property.
  /// If title_prop is set to true and no title is provided at render time, this will fallback to an existing title element in the svg if exists.
  pub title_prop: Option<bool>,

  /// Add desc tag via desc property.
  /// If desc_prop is set to true and no description is provided at render time, this will fallback to an existing desc element in the svg if exists.
  pub desc_prop: Option<bool>,

  /// All properties given to component will be forwarded on SVG tag.
  /// Possible values: "start", "end" or false.
  pub expand_props: ExpandProps,

  /// Keep `width` and `height` attributes from the root SVG tag.
  /// Removal is guaranteed if `dimensions: false`, unlike the `remove_dimensions: true` SVGO plugin option which also generates a `viewBox` from the dimensions if no `viewBox` is present.
  pub dimensions: Option<bool>,

  /// Replace SVG `width` and `height` by a custom value.
  /// If value is omitted, it uses `1em` in order to make SVG size inherits from text size.
  pub icon: Option<Icon>,

  /// Modify all SVG nodes with uppercase and use a specific template with `react-native-svg` imports.
  /// All unsupported nodes will be removed.
  pub native: Option<bool>,

  /// Add props to the root SVG tag.
  pub svg_props: Option<LinkedHashMap<String, String>>,

  /// Generates `.tsx` files with TypeScript typings.
  pub typescript: Option<bool>,

  /// Setting this to `true` will wrap the exported component in `React.memo`.
  pub memo: Option<bool>,

  /// Replace an attribute value by an other.
  /// The main usage of this option is to change an icon color to "currentColor" in order to inherit from text color.
  pub replace_attr_values: Option<HashMap<String, String>>,

  /// Specify a JSX runtime to use.
  /// * "classic": adds `import * as React from 'react'` on the top of file
  /// * "automatic": do not add anything
  /// * "classic-preact": adds `import { h } from 'preact'` on the top of file
  pub jsx_runtime: Option<JSXRuntime>,

  /// Specify a custom JSX runtime source to use. Allows to customize the import added at the top of generated file.
  pub jsx_runtime_import: Option<JSXRuntimeImport>,

  /// The named export defaults to `ReactComponent`, can be customized with the `named_export` option.
  pub named_export: String,

  /// If you prefer named export in any case, you may set the `export_type` option to `named`.
  pub export_type: Option<ExportType>,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      _ref: Default::default(),
      title_prop: Default::default(),
      desc_prop: Default::default(),
      expand_props: Default::default(),
      dimensions: Default::default(),
      icon: Default::default(),
      native: Default::default(),
      svg_props: Default::default(),
      typescript: Default::default(),
      memo: Default::default(),
      replace_attr_values: Default::default(),
      jsx_runtime: Default::default(),
      jsx_runtime_import: Default::default(),
      named_export: "ReactComponent".to_string(),
      export_type: Default::default(),
    }
  }
}
