use svgr_rs::{Caller, State};

#[napi(object, object_to_js = false)]
#[derive(Clone)]
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
#[derive(Clone)]
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
