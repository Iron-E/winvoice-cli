mod command;

use clap::Args as Clap;
use command::Command;

use super::retrieve::Args;

/// Delete data which is being stored by CLInvoice.
///
/// CLInvoice stores data which references other data. For example, an `Organization` exists in a
/// `Location`. So, if you attempt to delete any information which is being referenced by other
/// information (e.g. the `Location` of an `Organization`), this operation will fail.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[clap(about = "Retrieve information that was recorded with CLInvoice")]
pub struct Delete
{
	#[clap(flatten)]
	args: Args,

	#[clap(subcommand)]
	command: Command,
}
