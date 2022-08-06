use std::path::PathBuf;

use clap::Subcommand as Clap;
use clinvoice_export::Format;
use money2::Currency;

/// The specific type of information that is being retrieved.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RetrieveCommand
{
	/// Retrieve `Contact`s from the store (-s) specified.
	Contact,

	/// Retrieve `Employee`s from the store (-s) specified.
	Employee
	{
		/// Retrieve the `Employee` specified in the `id` field of the `[employees]` section of the
		/// CLInvoice configuration file.
		///
		/// Ignores --match.
		#[clap(action, group = "config", long, short)]
		default: bool,

		/// Set the `id` field of the `[employees]` section of the CLInvoice configuration file to
		/// the `Employee` which was retrieved by this operation.
		#[clap(action, group = "config", long, short)]
		set_default: bool,
	},

	/// Retrieve `Expense`s from the store (-s) specified.
	Expense,

	/// Retrieve `Job`s from the store (-s) specified.
	Job
	{
		/// Provide the curency to use when exporting.
		#[clap(default_value_t, long, short, requires("export"))]
		currency: Currency,

		/// Select a number of closed `Job`s and export them to a file.
		#[clap(action, long, short)]
		export: bool,

		/// What file format to --export to.
		///
		/// Supported formats are: markdown.
		#[clap(default_value_t = Format::Markdown, long, short, requires("export"))]
		format: Format,

		/// Which directory to --export files into.
		#[clap(long, short, requires("export"), value_name = "DIR", value_parser)]
		output_dir: Option<PathBuf>,
	},

	/// Retrieve `Location`s from the store (-s) specified.
	Location,

	/// Retrieve `Organization`s from the store (-s) specified.
	Organization
	{
		/// Retrieve the `Organization` specified in the `employer_id` field of the
		/// `[organizations]` section of the CLInvoice configuration file.
		///
		/// Ignores --match.
		#[clap(action, group = "config", long, short)]
		employer: bool,

		/// Set the `employer_id` field of the `[organizations]` section of the CLInvoice
		/// configuration file to the `Organization` which was retrieved by this operation.
		#[clap(action, group = "config", long, short)]
		set_employer: bool,
	},

	/// Retrieve `Timesheet`s from the store (-s) specified.
	Timesheet,
}
