mod as_ref;
mod command;
mod run_action;

use clap::Args as Clap;
pub use command::DeleteCommand;

use super::{match_args::MatchArgs, store_args::StoreArgs};
use crate::utils::{self, Identifiable};

/// Delete data which is being stored by CLInvoice.
///
/// CLInvoice stores data which references other data. For example, an `Organization` exists in a
/// `Location`. So, if you attempt to delete any information which is being referenced by other
/// information (e.g. the `Location` of an `Organization`), this operation will fail.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Delete
{
	/// The specifies the object to [`Delete`] and related arguments.
	#[clap(subcommand)]
	command: DeleteCommand,

	/// Specifies a file which can be used in place of the prompt of a user query.
	#[clap(flatten)]
	match_args: MatchArgs,

	/// Specifies the [`Store`](clinvoice_config::Store) to [`Delete`] from.
	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Delete
{
	/// Indicate with [`println!`] that a value of type `Deleted` — [`Display`]ed by calling
	/// `selector` on the `deleted` value — was deleted.
	pub(super) fn report_deleted<Deleted>(deleted: &Deleted)
	where
		Deleted: Identifiable,
	{
		utils::report_action("deleted", deleted);
	}
}
