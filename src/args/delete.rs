mod command;

use clap::Args as Clap;
use command::DeleteCommand;

use super::match_args::MatchArgs;

/// Delete data which is being stored by CLInvoice.
///
/// CLInvoice stores data which references other data. For example, an `Organization` exists in a
/// `Location`. So, if you attempt to delete any information which is being referenced by other
/// information (e.g. the `Location` of an `Organization`), this operation will fail.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Delete
{
	#[clap(subcommand)]
	command: DeleteCommand,

	#[clap(flatten)]
	match_args: MatchArgs,
}
