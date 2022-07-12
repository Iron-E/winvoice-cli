mod args;
mod command;

pub use args::Args;
use clap::Args as Clap;
use command::Command;

/// Retrieve information being stored by CLInvoice.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[clap(about = "Retrieve information that was recorded with CLInvoice")]
pub struct Retrieve
{
	#[clap(flatten)]
	args: Args,

	#[clap(subcommand)]
	command: Command,
}
