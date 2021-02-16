use structopt::StructOpt;

/// # Summary
///
/// The `clinvoice retrieve` subcommand.
///
/// # Remarks
///
/// ## Deleting
///
/// To delete an entity, the `--delete` flag must be passed. This will present a menu to select
/// which entities should be deleted, and then a confirmation dialogue will be presented to confirm
/// the deletions before performing the operation.
///
/// ## Updating
///
/// To update and update an entity, the `--update` flag must be passed. This will present a menu to
/// select which entities should be updateed, and then after a confirmation dialogue will be
/// presented to confirm the changes before update.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
#[structopt(name="retrieve", about="Retrieve information that was recorded with CLInvoice.")]
pub struct Retrieve
{
	/// # Summary
	///
	/// Whether or not to select retrieved entities for deletion.
	///
	/// # Remarks
	///
	/// Takes precedence over `--update`.
	#[structopt(short, long)]
	pub delete: bool,

	/// # Summary
	///
	/// Whether or not to select retrieved entities for updating.
	///
	/// # Remarks
	///
	/// `--delete` take precedence.
	#[structopt(short, long)]
	pub update: bool,

	/// # Summary
	///
	/// The retrieval command to perform.
	#[structopt(subcommand)]
	pub command: RetrieveCommand,
}

/// # Summary
///
/// The subcommand of the [`Retrieve`] command. Either `employee`, `job`, `location`,
/// `organization`, `person`.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructOpt)]
pub enum RetrieveCommand
{
	/// # Summary
	///
	/// The `clinvoice retrieve employee` subcommand.
	#[structopt(name="employee", about="Retrieve existing records about employees.")]
	Employee
	{
	},

	/// # Summary
	///
	/// The `clinvoice retrieve job` subcommand.
	#[structopt(name="job", about="Retrieve existing records about job.")]
	Job
	{
	},

	/// # Summary
	///
	/// The `clinvoice retrieve location` subcommand.
	#[structopt(name="location", about="Retrieve existing records about locations.")]
	Location
	{
	},

	/// # Summary
	///
	/// The `clinvoice retrieve organization` subcommand.
	#[structopt(name="organization", about="Retrieve existing records about organizations.")]
	Organization
	{
	},

	/// # Summary
	///
	/// The `clinvoice retrieve person` subcommand.
	#[structopt(name="person", about="Retrieve existing records about people.")]
	Person
	{
	},
}
