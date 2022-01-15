#![allow(clippy::suspicious_else_formatting)]

mod app;
mod dyn_result;
mod input;

// PERF: we're using `std::fs` because the main function does not need asynchrony at this point.
use std::{error::Error, fs, process};

use app::App;
use clinvoice_config::Config;
use dyn_result::DynResult;
use structopt::StructOpt;

/// # Summary
///
/// Exit `clinvoice` with status code 1, printing some `error`.
fn exit_with_err(error: impl Error) -> !
{
	if cfg!(debug_assertions)
	{
		panic!("{:?}", error)
	}

	eprintln!("\nERROR: {error}");
	process::exit(1)
}

/// # Summary
///
/// The main method.
#[tokio::main]
async fn main()
{
	// Create a default user configuration if not already present.
	Config::init().unwrap_or_else(|e| exit_with_err(e));

	// Get the user configuration.
	let config_bytes = fs::read(Config::path()).unwrap_or_else(|e| exit_with_err(e));
	let config: Config = toml::from_slice(&config_bytes).unwrap_or_else(|e| exit_with_err(e));

	// Run the CLInvoice application.
	App::from_args()
		.run(config)
		.await
		.unwrap_or_else(|e| exit_with_err(e.as_ref()));
}
