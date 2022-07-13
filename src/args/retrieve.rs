mod command;

use clap::Args as Clap;
use command::RetrieveCommand;

use super::match_args::MatchArgs;

/// Retrieve information being stored by CLInvoice.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Retrieve
{
	#[clap(subcommand)]
	command: RetrieveCommand,

	#[clap(flatten)]
	match_args: MatchArgs,
}
