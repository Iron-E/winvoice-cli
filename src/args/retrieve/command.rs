use clap::Subcommand as Clap;

/// The specific type of information that is being retrieved.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[clap(about = "Retrieve information that was recorded with CLInvoice")]
pub enum Command
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
