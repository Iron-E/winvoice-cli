mod command;

use clap::Args as Clap;
use command::UpdateCommand;

use super::match_args::MatchArgs;

/// Update information being stored by CLInvoice.
///
/// Sometimes information that is mistakenly entered into the CLInvoice system with incorrect
/// information, or information that has changed over time. This command will allow you to alter
/// the data that is being stored by CLInvoice to make it accurate.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Update
{
	#[clap(subcommand)]
	command: UpdateCommand,

	#[clap(flatten)]
	match_args: MatchArgs,
}
