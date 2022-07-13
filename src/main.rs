#![allow(clippy::suspicious_else_formatting)]
#![warn(missing_docs)]

mod app;
mod args;
mod dyn_result;
mod input;

use app::App;
use args::Args;
use clap::Parser;
use clinvoice_config::Config;
use dyn_result::DynResult;
use structopt::StructOpt;

/// # Summary
///
/// The main method.
#[tokio::main]
async fn main() -> DynResult<()>
{
	Args::parse().run().await
}
