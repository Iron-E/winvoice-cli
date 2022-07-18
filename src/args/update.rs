mod command;

use core::fmt::Display;

use clap::Args as Clap;
use clinvoice_config::Config;
use command::UpdateCommand;

use super::{match_args::MatchArgs, store_args::StoreArgs};
use crate::{utils, DynResult};

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
	/// Indicate with [`println!`] that a value of type `TUpdated` — [`Display`]ed by calling
	/// `selector` on the `created` value — was updated.
	pub(super) fn report_updated<TUpdated, TFn, TId>(updated: &TUpdated, selector: TFn)
	where
		TFn: FnOnce(&TUpdated) -> TId,
		TId: Display,
	{
		utils::report_action::<TUpdated, _>("updated", selector(updated));
	}

	/// Execute this command given the user's [`Config`].
	pub async fn run(self, config: &Config) -> DynResult<()>
	{
		let store = self.store_args.try_get_from(&config)?;
		todo!()
	}
}
