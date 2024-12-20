#![feature(let_chains, try_blocks)]

mod client;
mod command;
mod macros;
mod model;
mod version;

use anyhow::Result;
use clap::Parser;
use command::Search;

#[derive(Parser)]
#[command(
  name = "cli",
  about(None),
  long_about(None),
  disable_help_flag(true),
  disable_help_subcommand(true)
)]
enum Cli {
  Search(Search),
}

#[tokio::main]
async fn main() -> Result<()> {
  match Cli::parse() {
    Cli::Search(search) => search.run().await,
  }
}
