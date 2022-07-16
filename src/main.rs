#![allow(clippy::suspicious_else_formatting)]
#![warn(missing_docs)]

mod args;
mod dyn_result;
mod fmt;
mod input;

use args::Args;
use clap::Parser;
use dyn_result::DynResult;

/// # Summary
///
/// The main method.
#[tokio::main]
async fn main() -> DynResult<()>
{
	Args::parse().run().await
}
