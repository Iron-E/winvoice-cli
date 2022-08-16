use clap::Subcommand as Clap;
use clinvoice_schema::chrono::NaiveDateTime;

use crate::args::flag_or_argument::FlagOrArgument;

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
		///
		/// You may *optionally* provide the time that the job was finished (e.g.
		/// "2022-01-01T14:00:00").
		#[clap(
			default_missing_value = stringify!(true),
			default_value_t,
			group = "close-reopen",
			hide_default_value = true,
			long,
			short,
			value_name = "(OPTIONAL) DATE"
		)]
		close: FlagOrArgument<NaiveDateTime>,

		/// Select a number of `Job`s that have been export and sent to their respective clients,
		/// marking them as having been paid for.
		///
		/// You may *optionally* provide the time that the invoice was issued (e.g.
		/// "2022-01-01T14:00:00").
		#[clap(
			default_missing_value = stringify!(true),
			default_value_t,
			group = "issued-reopen",
			hide_default_value = true,
			long,
			short,
			value_name = "(OPTIONAL) DATE"
		)]
		invoice_issued: FlagOrArgument<NaiveDateTime>,

		/// Select a number of `Job`s that have been export and sent to their respective clients,
		/// marking them as having been paid for.
		///
		/// You may *optionally* provide the time that the invoice was paid (e.g.
		/// "2022-01-01T14:00:00").
		#[clap(
			default_missing_value = stringify!(true),
			default_value_t,
			group = "paid-reopen",
			hide_default_value = true,
			long,
			short = 'p',
			value_name = "(OPTIONAL) DATE"
		)]
		invoice_paid: FlagOrArgument<NaiveDateTime>,

		/// Select a number of `Job`s that were --closed in order to mark them as being currently
		/// worked on.
		#[clap(action, groups = &["close-reopen", "issued-reopen", "paid-reopen"], long, short)]
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
		///
		/// You may *optionally* provide the time that work started (e.g. "2022-01-01T14:00:00").
		#[clap(
			default_missing_value = stringify!(true),
			default_value_t,
			group = "quick-update",
			hide_default_value = true,
			long,
			short,
			value_name = "(OPTIONAL) DATE"
		)]
		restart: FlagOrArgument<NaiveDateTime>,

		/// Select a number of `Timesheet`s that are still being worked on, mark them as finished.
		///
		/// You may *optionally* provide the time that work ended (e.g. "2022-01-01T14:00:00").
		#[clap(
			default_missing_value = stringify!(true),
			default_value_t,
			group = "quick-update",
			hide_default_value = true,
			long,
			short,
			value_name = "(OPTIONAL) DATE"
		)]
		stop: FlagOrArgument<NaiveDateTime>,
	},
}
