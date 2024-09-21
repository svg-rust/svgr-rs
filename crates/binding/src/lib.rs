#![feature(path_file_prefix)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod config;
mod state;

use state::JsState;
use svgr_rs::{transform, Config};

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
