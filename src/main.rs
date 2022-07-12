#![allow(clippy::suspicious_else_formatting)]
#![warn(missing_docs)]

mod app;
mod dyn_result;
mod input;

// PERF: we're using `std::fs` because the main function does not need asynchrony at this point.
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
	let config = Config::read()?;
	App::from_args().run(config).await
}
