mod command;

use clap::Args as Clap;
use command::Command;

use super::retrieve::Args;

/// Update information being stored by CLInvoice.
///
/// Sometimes information that is mistakenly entered into the CLInvoice system with incorrect
/// information, or information that has changed over time. This command will allow you to alter
/// the data that is being stored by CLInvoice to make it accurate.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[clap(about = "Retrieve information that was recorded with CLInvoice")]
pub struct Update
{
	#[clap(flatten)]
	args: Args,

	#[clap(subcommand)]
	command: Command,
}
