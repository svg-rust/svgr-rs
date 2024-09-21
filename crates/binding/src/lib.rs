#![feature(path_file_prefix)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod config;
mod state;

use config::JsConfig;
use napi::{bindgen_prelude::AsyncTask, Env, JsString, Result, Task};
use state::JsState;
use svgr_rs::{transform, Config};

pub struct TransformTask {
  code: String,
  config: Option<JsConfig>,
  state: Option<JsState>,
}

impl Task for TransformTask {
  type Output = String;
  type JsValue = JsString;

  fn compute(&mut self) -> Result<Self::Output> {
    let config: Config = match self.config.clone() {
      Some(val) => val.try_into()?,
      None => Config::default(),
    };
    let state = self.state.clone().map(|s| s.into()).unwrap_or_default();
    match transform(self.code.clone(), config, state) {
      Ok(result) => napi::Result::Ok(result),
      Err(reason) => napi::Result::Err(napi::Error::from_reason(reason.to_string())),
    }
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_string(&output)
  }
}

#[napi(js_name = "transform")]
pub fn transform_node(
  code: String,
  config: Option<JsConfig>,
  state: Option<JsState>,
) -> AsyncTask<TransformTask> {
  AsyncTask::new(TransformTask {
    code,
    config,
    state,
  })
}
