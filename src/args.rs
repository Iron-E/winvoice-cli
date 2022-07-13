mod command;
mod create;
mod delete;
mod init;
mod match_args;
mod retrieve;
mod update;

use clap::Parser as Clap;
use command::Command;

/// CLInvoice is a tool to track and generate invoices from the command line. Pass --help for more.
///
/// It is capable of managing information about clients, employees, jobs, timesheets, and exporting
/// the information into the format of your choice.
#[derive(Clap, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Args
{
	/// A key from the `[stores]` section of the [configuration file](clinvoice_config::Config).
	#[clap(
		default_value = "default",
		help = "A key from the `[stores]` section of the configuration file.",
		long,
		short
	)]
	store: String,

	#[clap(subcommand)]
	command: Command,
}
