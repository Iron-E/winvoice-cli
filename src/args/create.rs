use clap::Subcommand as Clap;

/// Use CLInvoice to store new information.
///
/// CLInvoice is capable of storing multiple kinds of information. This command has multiple
/// subcommands and options which will guide you through the process and ensure that the data
/// provided is valid.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Create
{
	/// Create a `Contact` in the store (-s) specified.
	///
	/// You must always specify the --label, and may optionally specify that the `Contact` is an
	/// --address, --email, or --phone number. Passing none of those three arguments will perform
	/// less data verification before creating the `Contact`.
	Contact
	{
		/// The label of the `Contact` to be created.
		///
		/// e.g. "Office Phone", "Primary Email", "P.O. Box", "PayPal"
		#[clap(long, short)]
		label: String,

		/// The `Contact` to be created is the address of a physical `Location`.
		#[clap(action, group = "kind", group = "foo", long, short)]
		address: bool,

		/// The `Contact` to be created is an email address.
		#[clap(action, group = "kind", long, short)]
		email: bool,

		/// The `Contact` to be created is a phone number.
		#[clap(action, group = "kind", long, short)]
		phone: bool,

		/// The information which is represented by the --label.
		#[clap(
			default_value_if("address", Some("true"), Some("")),
			group = "foo",
			required_if_eq("email", "true"),
			required_if_eq("phone", "true")
		)]
		info: String,
	},

	/// Create a `Employee` in the store (-s) specified.
	Employee
	{
	},

	/// Create a `Expense` in the store (-s) specified.
	Expense,

	/// Create a `Job` in the store (-s) specified.
	Job,

	/// Create a `Location` in the store (-s) specified.
	Location,

	/// Create a `Organization` in the store (-s) specified.
	Organization,

	/// Create a `Timesheet` in the store (-s) specified.
	Timesheet,
}
