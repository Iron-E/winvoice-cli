mod command;

use clap::Args as Clap;
use clinvoice_config::Config;
use command::CreateCommand;

use super::store_args::StoreArgs;
use crate::DynResult;

/// Use CLInvoice to store new information.
///
/// CLInvoice is capable of storing multiple kinds of information. This command has multiple
/// subcommands and options which will guide you through the process and ensure that the data
/// provided is valid.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Create
{
	#[clap(subcommand)]
	command: CreateCommand,

	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Create
{
	pub async fn run(self, config: &Config) -> DynResult<()>
	{
		let store = self.store_args.try_get_from(config)?;
		todo!()
	}
}
