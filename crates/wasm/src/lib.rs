#![feature(once_cell_try_insert)]

mod search;

pub use search::search;
use serde::Deserialize;
use std::sync::OnceLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

static PLUGIN: OnceLock<Vec<Plugin>> = OnceLock::new();

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
  pub name: String,
  pub plugin_name: String,
  pub description: Option<String>,
  pub keywords: Vec<String>,
  pub official: bool,
  pub downloads: u64,
  pub recent_downloads: u64,
  pub default_version: String,
  pub repository: Option<String>,
  pub updated_at: String,
}

#[wasm_bindgen]
pub async fn init() -> Result<(), JsValue> {
  let request = Request::new_with_str("crates.json")?;
  let promise = web_sys::window()
    .unwrap()
    .fetch_with_request(&request);

  let json = JsFuture::from(promise)
    .await?
    .dyn_into::<Response>()?
    .json()
    .map(JsFuture::from)?
    .await?;

  let _ = PLUGIN.try_insert(serde_wasm_bindgen::from_value(json)?);

  Ok(())
}

fn plugins() -> Result<&'static [Plugin], JsError> {
  PLUGIN
    .get()
    .ok_or_else(|| JsError::new("plugin list is not initialized"))
    .map(Vec::as_slice)
}

#[wasm_bindgen]
pub fn get_plugins() -> Result<Vec<Plugin>, JsError> {
  Ok(plugins()?.to_vec())
}
