use crate::client::{self, CRATES_IO_API};
use crate::model::{Crate, CrateName};
use crate::version::VersionExt;
use anyhow::{Context, Result};
use clap::Args;
use itertools::Itertools;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::time::Instant;
use tokio::fs;

#[derive(Args)]
pub struct Search {
  #[arg(short, long)]
  include: Vec<String>,
  #[arg(short, long)]
  exclude: Vec<String>,
  #[arg(long, default_value_t = cfg!(debug_assertions))]
  pretty: bool,
}

impl Search {
  pub async fn run(mut self) -> Result<()> {
    let start = Instant::now();
    self.search_crates().await?;

    println!("done in {:?}", start.elapsed());

    Ok(())
  }

  #[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
  )]
  pub async fn search_crates(&mut self) -> Result<()> {
    let url = format!("{CRATES_IO_API}?q=tauri-plugin&per_page=100");
    let mut result = client::get::<SearchResult>(&url).await?;
    let mut next_page = result.meta.next_page.take();

    println!("found {} matching crates", result.meta.total);

    let mut current_page = 1;
    let total_pages = (result.meta.total as f64 / 100.0).ceil() as u64;
    print_page_download(current_page, total_pages);

    while let Some(mut page_url) = next_page {
      current_page += 1;
      print_page_download(current_page, total_pages);

      page_url.insert_str(0, CRATES_IO_API);
      let page = client::get::<SearchResult>(&page_url).await?;
      result.crates.extend(page.crates);
      next_page = page.meta.next_page;
    }

    let include = to_crate_name_set(&mut self.include);
    let exclude = to_crate_name_set(&mut self.exclude);

    let mut crates = result
      .crates
      .into_iter()
      .filter(|it| it.name.contains("tauri-plugin"))
      .filter(|it| !exclude.contains(&it.name))
      .filter(|it| !it.yanked)
      .filter(|it| it.max_stable_version.is_some())
      .filter(|it| !it.default_version.is_zero())
      .map(|it| (it.name.clone(), it))
      .collect::<HashMap<_, _>>();

    for krate in &include {
      if !crates.contains_key(krate) {
        let krate = Crate::fetch(krate).await?;
        crates.insert(krate.name.clone(), krate);
      }
    }

    for krate in crates.values_mut() {
      if let Err(err) = krate.update_fields().await {
        eprintln!("failed to update: {}\n{}\n", krate.name, err);
      }
    }

    let crates = crates
      .into_values()
      .sorted_unstable_by(|a, b| b.cmp(a))
      .collect_vec();

    write_crates(&crates, self.pretty)
      .await
      .with_context(|| "failed to write crates")
  }
}

#[derive(Deserialize)]
struct SearchResult {
  crates: Vec<Crate>,
  meta: SearchResultMeta,
}

#[derive(Deserialize)]
struct SearchResultMeta {
  total: u64,
  next_page: Option<String>,
}

async fn write_crates(crates: &[Crate], pretty: bool) -> Result<()> {
  let path = "web/static/crates.json";
  println!("writing crates to {path}");

  let contents = if pretty {
    serde_json::to_vec_pretty(crates)?
  } else {
    serde_json::to_vec(crates)?
  };

  println!("{} bytes written ({} crates)", contents.len(), crates.len());

  Ok(fs::write(path, contents).await?)
}

fn print_page_download(current: u64, total: u64) {
  println!("downloading page {current}/{total}");
}

fn to_crate_name_set(vec: &mut Vec<String>) -> HashSet<CrateName> {
  mem::take(vec)
    .into_iter()
    .map(CrateName::from)
    .collect()
}
