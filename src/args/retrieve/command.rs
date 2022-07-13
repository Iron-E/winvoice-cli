use clinvoice_export::Format;
use std::path::PathBuf;
use clap::Subcommand as Clap;

/// The specific type of information that is being retrieved.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[clap(about = "Retrieve information that was recorded with CLInvoice")]
pub enum RetrieveCommand
{
	/// Retrieve `Contact`s from the store (-s) specified.
	Contact,

	/// Retrieve `Employee`s from the store (-s) specified.
	Employee
	{
		/// Retrieve the `Employee` specified in the `id` field of the `[employees]` section of the
		/// CLInvoice configuration file.
		#[clap(action, default_value_t = false, long, short)]
		default: bool,

		/// Set the `id` field of the `[employees]` section of the CLInvoice configuration file to
		/// the `Employee` which was retrieved by this operation.
		#[clap(action, default_value_t = false, long, short)]
		set_default: bool,
	},

	/// Retrieve `Expense`s from the store (-s) specified.
	Expense,

	/// Retrieve `Job`s from the store (-s) specified.
	Job
	{
		/// Select retrieved `Job`s and export them to a file.
		#[clap(action, default_value_t = false, long, short)]
		export: bool,

		/// What file format to `--export` to.
		#[clap(long, short, requires("export"), value_name = "markdown")]
		format: Option<Format>,

		/// What directory the `--export`ed files should be placed in.
		#[clap(long, short, requires("export"), value_name = "DIR", value_parser)]
		output_dir: Option<PathBuf>,
	},

	/// Retrieve `Location`s from the store (-s) specified.
	Location,

	/// Retrieve `Organization`s from the store (-s) specified.
	Organization
	{
		/// Retrieve the `Organization` specified in the `employer_id` field of the `[organizations]`
		/// section of the CLInvoice configuration file.
		#[clap(action, default_value_t = false, long, short)]
		employer: bool,

		/// Set the `employer_id` field of the `[organizations]` section of the CLInvoice configuration
		/// file to the `Organization` which was retrieved by this operation.
		#[clap(action, default_value_t = false, long, short)]
		set_employer: bool,
	},

	/// Retrieve `Timesheet`s from the store (-s) specified.
	Timesheet,
}
