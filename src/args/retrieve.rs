mod command;

use clap::Args as Clap;
use clinvoice_config::Config;
use command::RetrieveCommand;

use super::{match_args::MatchArgs, store_args::StoreArgs};
use crate::DynResult;

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

impl Retrieve
{
	pub async fn run(self, config: Config) -> DynResult<()>
	{
		let store = self.store_args.try_get_from(&config).cloned()?;
		todo!()
	}
}
