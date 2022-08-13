mod as_ref;
mod command;
mod run_action;

use clap::Args as Clap;
pub use command::CreateCommand;

use super::store_args::StoreArgs;
use crate::utils::{self, Identifiable};

/// Use CLInvoice to store new information.
///
/// CLInvoice is capable of storing multiple kinds of information. This command has multiple
/// subcommands and options which will guide you through the process and ensure that the data
/// provided is valid.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Create
{
	/// The object to [`Create`] and related arguments.
	#[clap(subcommand)]
	command: CreateCommand,

	/// Specifies the [`Store`](clinvoice_config::Store) to insert [`Create`]d data into.
	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Create
{
	/// Indicate with [`println!`] that a value of type `Created` — [`Display`]ed by calling
	/// `selector` on the `created` value — was created.
	pub(super) fn report_created<Created>(created: &Created)
	where
		Created: Identifiable,
	{
		utils::report_action("created", created);
	}
}
