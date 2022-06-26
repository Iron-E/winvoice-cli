#![allow(clippy::suspicious_else_formatting)]

mod app;
mod dyn_result;
mod input;

// PERF: we're using `std::fs` because the main function does not need asynchrony at this point.
use std::fs;

use app::App;
use clinvoice_config::Config;
use dyn_result::DynResult;
use structopt::StructOpt;

/// # Summary
///
/// The main method.
#[tokio::main]
async fn main() -> DynResult<()>
{
	// Create a default user configuration if not already present.
	Config::init()?;

	// Get the user configuration.
	let config_bytes = fs::read(Config::path())?;
	let config: Config = toml::from_slice(&config_bytes)?;

	// Run the CLInvoice application.
	App::from_args().run(config).await
}
