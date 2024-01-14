#![feature(path_file_prefix)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use svgr_rs::{transform, Config, State};
use swc_core::node::get_deserialized;

#[napi(js_name = "transform")]
pub async fn transform_node(
  code: String,
  config: napi::bindgen_prelude::Buffer,
  state: Option<State>,
) -> napi::bindgen_prelude::Result<String> {
  let config: Config = get_deserialized(&config)?;
  let state = match state {
    Some(state) => state,
    None => Default::default(),
  };
  match transform(code, config, state) {
    Ok(result) => napi::bindgen_prelude::Result::Ok(result),
    Err(reason) => napi::bindgen_prelude::Result::Err(napi::bindgen_prelude::Error::from_reason(
      reason.to_string(),
    )),
  }
}
