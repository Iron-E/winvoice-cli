mod app;
mod io;

use
{
	app::App,
	clinvoice_config::Config,
	std::fs,
	structopt::StructOpt,
};

/// # Summary
///
/// The main method.
fn main()
{
	// Create a default user configuration if not already present.
	Config::init().unwrap();

	// Get the user configuration.
	let config_bytes = fs::read(Config::path()).unwrap();
	let config: Config = toml::from_slice(&config_bytes).unwrap();

	// Run the CLInvoice application.
	App::from_args().run(config).unwrap();
}
