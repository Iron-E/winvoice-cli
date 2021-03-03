mod app;
mod dyn_result;
mod io;

use
{
	app::App,
	clinvoice_config::Config,
	dyn_result::DynResult,
	structopt::StructOpt,
	std::{error::Error, fs, process},
};

/// # Summary
///
/// Exit `clinvoice` with status code 1, printing some `error`.
fn exit_with_err<E>(error: E) -> ! where E : Error
{
	if cfg!(debug_assertions) { panic!("{:?}", error) }

	eprintln!("\nERROR: {}", error);
	process::exit(1)
}

/// # Summary
///
/// The main method.
fn main()
{
	// Create a default user configuration if not already present.
	Config::init().unwrap_or_else(|e| exit_with_err(e));

	// Get the user configuration.
	let config_bytes = fs::read(Config::path()).unwrap_or_else(|e| exit_with_err(e));
	let config: Config = toml::from_slice(&config_bytes).unwrap_or_else(|e| exit_with_err(e));

	// Run the CLInvoice application.
	App::from_args().run(config).unwrap_or_else(|e| exit_with_err(e.as_ref()));
}
