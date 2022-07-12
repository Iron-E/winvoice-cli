use clap::Subcommand as Clap;

/// Use CLInvoice to store new information.
///
/// CLInvoice is capable of storing multiple kinds of information. This command has multiple
/// subcommands and options which will guide you through the process and ensure that the data
/// provided is valid.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[clap(about = "Retrieve information that was recorded with CLInvoice")]
pub enum Create
{
	// TODO: flesh out
	Contact,
	Employee,
	Expense,
	Job,
	Location,
	Organization,
	Timesheet,
}
