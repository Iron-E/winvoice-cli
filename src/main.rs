mod app;
mod io;

use
{
	app::App,
	clinvoice_adapter::DynamicResult,
	clinvoice_config::Config,
	std::fs,
	structopt::StructOpt,
};

/// # Summary
///
/// The main method.
fn main() -> DynamicResult<()>
{
	// Get the user configuration.
	Config::init()?;
	let config_bytes = fs::read(Config::path())?;
	let config = toml::from_slice(&config_bytes)?;

	// Run the CLInvoice application.
	App::from_args().run(config)?;

	// Return OK if the app hasn't died yet.
	return Ok(());
}
