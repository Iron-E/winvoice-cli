use clap::Subcommand as Clap;

/// The specific type of information that is being updated.
#[derive(Clap, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UpdateCommand
{
	/// Update `Contact`s in the store (-s) specified.
	Contact,

	/// Update `Employee`s in the store (-s) specified.
	Employee
	{
		/// Update the `Employee` specified in the `id` field of the `[employees]` section of the
		/// CLInvoice configuration file.
		///
		/// Ignores --match.
		#[clap(action, long, short)]
		default: bool,
	},

	/// Update `Expense`s in the store (-s) specified.
	Expense,

	/// Update `Job`s in the store (-s) specified.
	Job
	{
		/// Select a number of `Job`s that are currently being worked on, in order to mark them as
		/// having been completed.
		#[clap(action, group = "quick-update", long, short)]
		close: bool,

		/// Select a number of `Job`s that have been export and sent to their respective clients,
		/// marking them as having been paid for.
		#[clap(action, group = "quick-update", long, short)]
		invoice_issued: bool,

		/// Select a number of `Job`s that have been export and sent to their respective clients,
		/// marking them as having been paid for.
		#[clap(action, group = "quick-update", long, short)]
		invoice_paid: bool,

		/// Select a number of `Job`s that were --closed in order to mark them as being currently
		/// worked on.
		#[clap(action, group = "quick-update", long, short)]
		reopen: bool,
	},

	/// Update `Location`s in the store (-s) specified.
	Location,

	/// Update `Organization`s in the store (-s) specified.
	Organization
	{
		/// Update the `Organization` specified in the `employer_id` field of the `[organizations]`
		/// section of the CLInvoice configuration file.
		///
		/// Ignores --match.
		#[clap(action, long, short)]
		employer: bool,
	},

	/// Update `Timesheet`s in the store (-s) specified.
	Timesheet
	{
		/// Select a number of `Timesheet`s that were marked finished, and make them active again.
		#[clap(action, group = "quick-update", long, short)]
		restart: bool,

		/// Select a number of `Timesheet`s that are still being worked on, mark them as finished.
		#[clap(action, group = "quick-update", long, short)]
		stop: bool,
	},
}
