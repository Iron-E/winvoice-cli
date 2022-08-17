use core::time::Duration;
use std::path::PathBuf;

use clap::Subcommand as Clap;
use clinvoice_schema::chrono::NaiveDateTime;
use money2::Money;

use crate::args::flag_or_argument::FlagOrArgument;

/// Use CLInvoice to store new information.
///
/// CLInvoice is capable of storing multiple kinds of information. This command has multiple
/// subcommands and options which will guide you through the process and ensure that the data
/// provided is valid.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CreateCommand
{
	/// Create a `Contact` in the store (-s) specified.
	///
	/// You must always specify the --label, and may optionally specify that the `Contact` is an
	/// --address, --email, or --phone number. Passing none of those three arguments will perform
	/// less data verification before creating the `Contact`.
	///
	/// See the documentation for more information about `Contact`s.
	Contact
	{
		/// The `label` of the `Contact` to create.
		///
		/// e.g. "Office Phone", "Primary Email", "P.O. Box", "PayPal"
		#[clap(long, short)]
		label: String,

		/// The `Contact` to create is the address of a physical `Location`.
		///
		/// You may *optionally* provide a path to a YAML file that contains a valid match
		/// condition/query/search for a CLInvoice location.
		#[clap(
			default_missing_value = stringify!(true),
			default_value_t,
			groups = &["content", "kind"],
			hide_default_value = true,
			long,
			short,
			value_name = "(OPTIONAL) FILE",
			value_parser
		)]
		address: FlagOrArgument<PathBuf>,

		/// The `Contact` to create is an email address.
		#[clap(action, group = "kind", long, short)]
		email: bool,

		/// The `Contact` to create is a phone number.
		#[clap(action, group = "kind", long, short)]
		phone: bool,

		/// The information which is represented by the --label.
		#[clap(
			default_value_if("address", None, Some("")),
			group = "content",
			required_if_eq("email", stringify!(true)),
			required_if_eq("phone", stringify!(true))
		)]
		info: String,
	},

	/// Create a `Employee` in the store (-s) specified.
	///
	/// See the documentation for more information about `Employees`s.
	Employee
	{
		/// The `name` of the `Employee` to create.
		#[clap(long, short)]
		name: String,

		/// The `status` of the `Employee` to create e.g. Contracted, Employed, Resigned
		#[clap(long, short)]
		status: String,

		/// The `title` of the `Employee` to create e.g. Developer, CEO, Manager
		#[clap(long, short)]
		title: String,
	},

	/// Create a `Expense` in the store (-s) specified.
	///
	/// See the documentation for more information about `Expense`s.
	Expense
	{
		/// The `category` of `Expense` e.g. Food, Travel
		#[clap(long, short)]
		category: String,

		/// The `cost` of the `Expense` to create e.g. "50.00 USD"
		#[clap(long, short = '$')]
		cost: Money,

		/// A specific `description` of the `Expense` to create e.g. "Flight to Meeting"
		#[clap(long, short)]
		description: String,

		/// A path to a YAML file that contains a valid match condition/query/search for a
		/// CLInvoice Timesheet.
		#[clap(long, short, value_name = "FILE", value_parser)]
		timesheet: Option<PathBuf>,
	},

	/// Create a `Job` in the store (-s) specified.
	///
	/// See the documentation for more information about `Job`s.
	Job
	{
		/// A path to a YAML file that contains a valid match condition/query/search for a
		/// CLInvoice Organization.
		#[clap(group = "client-args", long, short, value_name = "FILE", value_parser)]
		client: Option<PathBuf>,

		/// The date and time that work on the `Job` to create stopped.
		///
		/// See --date-open for formatting information.
		#[clap(long, requires("date-open"))]
		date_close: Option<NaiveDateTime>,

		/// The date and time that the `Job` to create's associated `Invoice` was issued to the
		/// `client`.
		///
		/// See --date-open for formatting information.
		#[clap(long, requires("date-close"))]
		date_invoice_issued: Option<NaiveDateTime>,

		/// The date and time that the `Job` to create's associated `Invoice` was paid by the
		/// `client`.
		///
		/// See --date-open for formatting information.
		#[clap(long, requires("date-invoice-issued"))]
		date_invoice_paid: Option<NaiveDateTime>,

		/// The date and time that work on the `Job` to create started. Defaults to the current
		/// date and time.
		///
		/// e.g. December 12th, 2022 at 1:30:00pm is "2022-12-31T13:30:00"
		#[clap(long)]
		date_open: Option<NaiveDateTime>,

		/// Set the `client` to the `Organization` specified by the `employee` field of the
		/// `[organizations]` section of the CLInvoice config.
		#[clap(action, group = "client-args", long, short)]
		employer: bool,

		/// The `invoice.hourly_rate` of the `Job` to create e.g. "50.00 USD".
		#[clap(long, short = '$')]
		hourly_rate: Money,

		/// The `increment` of the `Job` to create e.g. "15min".
		///
		/// If this argument is not provided, CLInvoice will attempt to use the value from the
		/// `default_increment` key in the `[jobs]` field of your configuration.
		///
		/// See the documentation of [`humantime`] to see more information about how to format
		/// this argument.
		#[clap(long, short, value_parser = humantime::parse_duration)]
		increment: Option<Duration>,

		/// The `notes` of the `Job` to create.
		#[clap(default_value_t, long, short)]
		notes: String,

		/// The `objectives` of the `Job` to create.
		#[clap(long, short)]
		objectives: String,
	},

	/// Create one or more `Location`s in the store (-s) specified.
	///
	/// See the documentation for more information about `Job`s.
	///
	/// Example: `clinvoice create location Phoenix --outside Arizona USA --inside`
	Location
	{
		/// Indicate that final location <NAME> specified is inside another `Location`.
		///
		/// You may *optionally* provide a path to a YAML file that contains a valid match
		/// condition/query/search for a CLInvoice location.
		#[clap(
			default_missing_value = stringify!(true),
			default_value_t,
			hide_default_value = true,
			long,
			short,
			value_name = "(OPTIONAL) FILE",
			value_parser
		)]
		inside: FlagOrArgument<PathBuf>,

		/// The `name`s of the locations which will be created, in order of innermost to outermost.
		#[clap(required(true))]
		names: Vec<String>,

		/// Indicate that first location <NAME> specified is outside another `Location`.
		///
		/// You may *optionally* provide a path to a YAML file that contains a valid match
		/// condition/query/search for a CLInvoice location.
		#[clap(
			default_missing_value = stringify!(true),
			default_value_t,
			hide_default_value = true,
			long,
			short,
			value_name = "(OPTIONAL) FILE",
			value_parser
		)]
		outside: FlagOrArgument<PathBuf>,
	},

	/// Create a `Organization` in the store (-s) specified.
	///
	/// See the documentation for more information about `Organization`s.
	Organization
	{
		/// A path to a YAML file that contains a valid match condition/query/search for a
		/// CLInvoice Organization.
		#[clap(long, short, value_name = "FILE", value_parser)]
		location: Option<PathBuf>,

		/// The `name` of the `Organization` to create.
		#[clap(long, short)]
		name: String,
	},

	/// Create a `Timesheet` in the store (-s) specified.
	///
	/// See the documentation for more information about `Timesheet`s.
	Timesheet
	{
		/// Set the one who is working on the `Timesheet` to the `Employee` specified by the `id`
		/// field of the `[employees]` section of the CLInvoice config.
		#[clap(action, group = "employee-args", long, short)]
		default_employee: bool,

		/// A path to a YAML file that contains a valid match condition/query/search for a
		/// CLInvoice Employee.
		#[clap(group = "employee-args", long, short, value_name = "FILE", value_parser)]
		employee: Option<PathBuf>,

		/// A path to a YAML file that contains a valid match condition/query/search for a
		/// CLInvoice Job.
		#[clap(long, short, value_name = "FILE", value_parser)]
		job: Option<PathBuf>,

		/// The `time_begin` of the `Timesheet` to create. Defaults to the current date and time.
		///
		/// e.g. December 12th, 2022 at 1:30:00pm is "2022-12-31T13:30:00"
		#[clap(long)]
		time_begin: Option<NaiveDateTime>,

		/// The `time_end` of the `Timesheet` to create. Defaults to the current time.
		///
		/// See --time-begin for formatting information.
		#[clap(long, requires("time-begin"))]
		time_end: Option<NaiveDateTime>,

		/// The `work_notes` of the `Timesheet` to create.
		#[clap(long, short)]
		work_notes: Option<String>,
	},
}
