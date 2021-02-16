use structopt::StructOpt;

/// # Summary
///
/// The `clinvoice create` subcommand.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(name="create", about="Record information information with CLInvoice.")]
pub enum Create
{
	/// # Summary
	///
	/// The `clinvoice create employee` subcommand.
	#[structopt(name="employee", about="Create a new employee record.")]
	Employee
	{
	},

	/// # Summary
	///
	/// The `clinvoice create job` subcommand.
	#[structopt(name="employee", about="Create a new job record.")]
	Job
	{
	},

	/// # Summary
	///
	/// The `clinvoice create location` subcommand.
	#[structopt(name="employee", about="Create a new location record.")]
	Location
	{
	},

	/// # Summary
	///
	/// The `clinvoice create organization` subcommand.
	#[structopt(name="employee", about="Create a new organization record.")]
	Organization
	{
	},

	/// # Summary
	///
	/// The `clinvoice create person` subcommand.
	#[structopt(name="employee", about="Create a new organization record.")]
	Person
	{
	},
}
