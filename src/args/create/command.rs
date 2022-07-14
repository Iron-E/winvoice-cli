use clap::Subcommand as Clap;
use clinvoice_config::{Config, Store};
use clinvoice_finance::Money;
use clinvoice_schema::chrono::NaiveDateTime;
use humantime::Duration;
use sqlx::{Database, Pool};

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
		#[clap(action, group = "kind", group = "content", long, short)]
		address: bool,

		/// The `Contact` to create is an email address.
		#[clap(action, group = "kind", long, short)]
		email: bool,

		/// The `Contact` to create is a phone number.
		#[clap(action, group = "kind", long, short)]
		phone: bool,

		/// The information which is represented by the --label.
		#[clap(
			default_value_if("address", Some("true"), Some("")),
			group = "content",
			required_if_eq("email", "true"),
			required_if_eq("phone", "true")
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
	},

	/// Create a `Job` in the store (-s) specified.
	///
	/// See the documentation for more information about `Job`s.
	Job
	{
		/// The `date_close` of the `Job` to create.
		#[clap(long, requires("date-open"), short = 'c')]
		date_close: Option<NaiveDateTime>,

		/// The `invoice.date_issued` of the `Job` to create.
		#[clap(long, short = 's')]
		date_invoice_issued: Option<NaiveDateTime>,

		/// The `invoice.date_paid` of the `Job` to create.
		#[clap(long, requires("date-invoice-issued"), short = 'p')]
		date_invoice_paid: Option<NaiveDateTime>,

		/// The `date_open` of the `Job` to create.
		#[clap(long, short = 'o')]
		date_open: Option<NaiveDateTime>,

		/// The `invoice.hourly_rate` of the `Job` to create e.g. "50.00 USD".
		#[clap(long, short = '$')]
		hourly_rate: Money,

		/// The `increment` of the `Job` to create e.g. "15min".
		///
		/// See the documentation of [`humantime`] to see more information about how to format
		/// this argument.
		#[clap(long, short)]
		increment: Duration,

		/// The `notes` of the `Job` to create.
		#[clap(long, short)]
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
		#[clap(action, long, short)]
		inside: bool,

		/// Indicate that first location <NAME> specified is outside another `Location`.
		#[clap(action, long, short)]
		outside: bool,

		/// The `name`s of the locations which will be created, in order of innermost to outermost.
		#[clap(required(true))]
		names: Vec<String>,
	},

	/// Create a `Organization` in the store (-s) specified.
	///
	/// See the documentation for more information about `Organization`s.
	Organization
	{
		/// The `name` of the `Organization` to create.
		#[clap(long, short)]
		name: String,
	},

	/// Create a `Timesheet` in the store (-s) specified.
	///
	/// See the documentation for more information about `Timesheet`s.
	Timesheet
	{
		/// Set the one who is working on the `Timesheet` to the `Employee` specified by the `id` field of
		/// the `[employees]` section of the CLInvoice config.
		#[clap(action, long, short)]
		default_employee: bool,

		/// The `time_begin` of the `Timesheet` to create. Defaults to the current time.
		///
		/// e.g. December 12th, 2022 at 1:30:00pm is "2022-12-31T13:30:00"
		#[clap(long, short = 'b')]
		time_begin: Option<NaiveDateTime>,

		/// The `time_end` of the `Timesheet` to create. Defaults to the current time.
		///
		/// See --time-begin for more info.
		#[clap(long, requires("time-begin"), short = 'e')]
		time_end: Option<NaiveDateTime>,

		/// The `work_notes` of the `Timesheet` to create.
		#[clap(long, short)]
		work_notes: Option<String>,
	},
}
