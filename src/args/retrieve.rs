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
	#[clap(subcommand)]
	command: RetrieveCommand,

	#[clap(flatten)]
	match_args: MatchArgs,

	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Retrieve
{
	pub async fn run(self, config: Config) -> DynResult<()>
	{
		todo!()
	}
}
