#![feature(path_file_prefix)]

#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use swc_core::node::get_deserialized;
use svgr_rs::{transform as transform_core, Config, State};

#[napi]
pub async fn transform(code: String, config: napi::bindgen_prelude::Buffer, state: Option<State>) -> napi::bindgen_prelude::Result<String> {
    let config: Config = get_deserialized(&config)?;
    match transform_core(code, config, state) {
        Ok(result) => napi::bindgen_prelude::Result::Ok(result),
        Err(reason) => napi::bindgen_prelude::Result::Err(napi::bindgen_prelude::Error::from_reason(reason)),
    }
}
