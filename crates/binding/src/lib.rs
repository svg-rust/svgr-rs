#![feature(path_file_prefix)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use svgr_rs::{transform, Caller, Config, State};
use swc_core::node::get_deserialized;

#[napi(object, object_to_js = false)]
pub struct JsCaller {
  pub name: Option<String>,
  pub previous_export: Option<String>,
}

impl From<JsCaller> for Caller {
  fn from(val: JsCaller) -> Self {
    Self {
      name: val.name,
      previous_export: val.previous_export,
    }
  }
}

#[napi(object, object_to_js = false)]
pub struct JsState {
  pub file_path: Option<String>,
  pub component_name: Option<String>,
  pub caller: Option<JsCaller>,
}

impl From<JsState> for State {
  fn from(val: JsState) -> Self {
    Self {
      file_path: val.file_path,
      component_name: val.component_name,
      caller: val.caller.map(|c| c.into()),
    }
  }
}

#[napi(js_name = "transform")]
pub async fn transform_node(
  code: String,
  config: napi::bindgen_prelude::Buffer,
  js_state: Option<JsState>,
) -> napi::bindgen_prelude::Result<String> {
  let config: Config = get_deserialized(&config)?;
  let state = js_state.map(|s| s.into()).unwrap_or_default();
  match transform(code, config, state) {
    Ok(result) => napi::bindgen_prelude::Result::Ok(result),
    Err(reason) => napi::bindgen_prelude::Result::Err(napi::bindgen_prelude::Error::from_reason(
      reason.to_string(),
    )),
  }
}
