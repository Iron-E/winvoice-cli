use clap::Subcommand as Clap;

/// The specific type of information that is being deleted.
#[derive(Clap, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeleteCommand
{
	/// Delete `Contact`s in the store (-s) specified.
	Contact,

	/// Delete `Employee`s in the store (-s) specified.
	Employee,

	/// Delete `Expense`s in the store (-s) specified.
	Expense,

	/// Delete `Job`s in the store (-s) specified.
	Job,

	/// Delete `Location`s in the store (-s) specified.
	Location,

	/// Delete `Organization`s in the store (-s) specified.
	Organization,

	/// Delete `Timesheet`s in the store (-s) specified.
	Timesheet,
}
