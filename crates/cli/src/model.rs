use crate::client::{self, CRATES_IO, CRATES_IO_API};
use crate::return_if_ne;
use anyhow::{anyhow, Result};
use colored::Colorize;
use regex::Regex;
use semver::Version;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_value, Value as JsonValue};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, LazyLock};

static PLUGIN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"tauri-plugin-?").unwrap());

#[derive(Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Crate {
  pub name: CrateName,
  pub description: Option<String>,
  pub keywords: Option<Vec<String>>,
  pub downloads: u64,
  pub recent_downloads: u64,
  pub default_version: Version,
  pub repository: Option<String>,
  pub updated_at: String,

  // Custom fields.
  #[serde(default)]
  pub plugin_name: String,
  #[serde(default)]
  pub official: bool,

  // Should not be serialized.
  #[serde(skip_serializing)]
  pub links: CrateLink,
  #[serde(skip_serializing)]
  pub max_stable_version: Option<Version>,
  #[serde(skip_serializing)]
  pub yanked: bool,
}

impl Crate {
  pub async fn fetch(name: &str) -> Result<Self> {
    let url = format!("{CRATES_IO_API}/{name}");
    client::get::<JsonValue>(&url)
      .await?
      .get_mut("crate")
      .map(|it| from_value::<Crate>(it.take()))
      .ok_or_else(|| anyhow!("crate not found: {name}"))?
      .map_err(Into::into)
  }

  pub async fn update_fields(&mut self) -> Result<()> {
    println!("{}", format!("updating {}", self.name).cyan());

    self.links.set_origin();
    self.set_plugin_name();

    if self.keywords.is_none() {
      self.keywords = Some(
        Self::fetch(&self.name)
          .await?
          .keywords
          .unwrap_or_default(),
      );
    }

    let owner = self
      .links
      .owner_user
      .as_ref()
      .ok_or_else(|| anyhow!("crate {} has no owner", self.name))?;

    self.official = client::get::<JsonValue>(owner)
      .await?
      .get_mut("users")
      .and_then(|it| from_value::<Vec<User>>(it.take()).ok())
      .ok_or_else(|| anyhow!("failed to get users for {}", self.name))?
      .iter()
      .any(|user| user.login == "tauri-bot");

    Ok(())
  }

  fn set_plugin_name(&mut self) {
    let name = &self.name;
    self.plugin_name = PLUGIN_RE.replace(name, "").to_string();
  }
}

impl PartialEq for Crate {
  fn eq(&self, other: &Self) -> bool {
    self.name.eq(&other.name)
  }
}

impl Eq for Crate {}

impl PartialOrd for Crate {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Crate {
  fn cmp(&self, other: &Self) -> Ordering {
    return_if_ne!(self.recent_downloads.cmp(&other.recent_downloads));
    return_if_ne!(self.downloads.cmp(&other.downloads));
    self.name.cmp(&other.name)
  }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CrateLink {
  pub owner_user: Option<String>,
}

impl CrateLink {
  pub fn set_origin(&mut self) {
    if let Some(owner_user) = &mut self.owner_user {
      owner_user.insert_str(0, CRATES_IO);
    }
  }
}

pub struct CrateName(Arc<str>);

impl PartialEq for CrateName {
  fn eq(&self, other: &Self) -> bool {
    self.0.eq(&other.0)
  }
}

impl Eq for CrateName {}

impl Hash for CrateName {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}

impl Clone for CrateName {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

impl fmt::Display for CrateName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl Deref for CrateName {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Serialize for CrateName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for CrateName {
  fn deserialize<D>(deserializer: D) -> Result<CrateName, D::Error>
  where
    D: Deserializer<'de>,
  {
    let inner = String::deserialize(deserializer)?;
    Ok(CrateName(Arc::from(inner)))
  }
}

impl From<String> for CrateName {
  fn from(value: String) -> Self {
    Self(Arc::from(value))
  }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct User {
  login: String,
}
