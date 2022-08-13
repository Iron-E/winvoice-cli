mod as_ref;
mod command;
mod run_action;

use clap::Args as Clap;
pub use command::UpdateCommand;

use super::{match_args::MatchArgs, store_args::StoreArgs};
use crate::utils::{self, Identifiable};

/// Update information being stored by CLInvoice.
///
/// Sometimes information that is mistakenly entered into the CLInvoice system with incorrect
/// information, or information that has changed over time. This command will allow you to alter
/// the data that is being stored by CLInvoice to make it accurate.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Update
{
	/// Specifies the object to [`Update`] and related arguments.
	#[clap(subcommand)]
	command: UpdateCommand,

	/// Specifies a file which can be used in place of the prompt of a user query.
	#[clap(flatten)]
	match_args: MatchArgs,

	/// Specifies the [`Store`](clinvoice_config::Store) to send [`Update`]s to.
	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Update
{
	/// Indicate with [`println!`] that a value of type `Updated` — [`Display`]ed by calling
	/// `selector` on the `created` value — was updated.
	pub(super) fn report_updated<Updated>(updated: &Updated)
	where
		Updated: Identifiable,
	{
		utils::report_action("updated", updated);
	}
}
