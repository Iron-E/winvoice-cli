use clap::Subcommand as Clap;

/// The specific type of information that is being updated.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[clap(about = "Retrieve information that was recorded with CLInvoice")]
pub enum UpdateCommand
{
	/// Update `Contact`s in the store (-s) specified.
	Contact,

	/// Update `Employee`s in the store (-s) specified.
	Employee,

	/// Update `Expense`s in the store (-s) specified.
	Expense,

	/// Update `Job`s in the store (-s) specified.
	Job
	{
		/// Select a number of `Job`s that are currently being worked on, in order to mark them as
		/// having been completed.
		#[clap(action, default_value_t = false, group = "quick-update", long, short)]
		close: bool,

		/// Select a number of `Job`s that have been export and sent to their respective clients,
		/// marking them as having been paid for.
		#[clap(action, default_value_t = false, group = "quick-update", long, short)]
		paid: bool,

		/// Select a number of `Job`s that were --closed in order to mark them as being currently
		/// worked on.
		#[clap(action, default_value_t = false, group = "quick-update", long, short)]
		reopen: bool,
	},

	/// Update `Location`s in the store (-s) specified.
	Location,

	/// Update `Organization`s in the store (-s) specified.
	Organization,

	/// Update `Timesheet`s in the store (-s) specified.
	Timesheet
	{
		/// Select a number of `Timesheet`s that were marked finished, and make them active again.
		#[clap(action, default_value_t = false, group = "quick-update", long, short)]
		restart: bool,

		/// Select a number of `Timesheet`s that are still being worked on, mark them as finished.
		#[clap(action, default_value_t = false, group = "quick-update", long, short)]
		stop: bool,
	},
}
