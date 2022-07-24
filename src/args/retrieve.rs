mod as_ref;
mod command;
mod run_action;

use clap::Args as Clap;
use command::RetrieveCommand;

use super::{match_args::MatchArgs, store_args::StoreArgs};

/// Retrieve information being stored by CLInvoice.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Retrieve
{
	/// The specific object to [`Retrieve`] and related arguments.
	#[clap(subcommand)]
	command: RetrieveCommand,

	/// Specifies a file which can be used in place of the prompt of a user query.
	#[clap(flatten)]
	match_args: MatchArgs,

	/// Specifies the [`Store`](clinvoice_config::Store) to [`Retrieve`] from.
	#[clap(flatten)]
	store_args: StoreArgs,
}
