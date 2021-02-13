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
pub struct Retrieve
{
	/// # Summary
	///
	/// Whether or not to select retrieved entities for deletion.
	///
	/// # Remarks
	///
	/// Takes precedence over `--update`.
	pub delete: bool,

	/// # Summary
	///
	/// Whether or not to select retrieved entities for updating.
	///
	/// # Remarks
	///
	/// `--delete` take precedence.
	pub update: bool,

	/// # Summary
	///
	/// The retrieval command to perform.
	pub command: RetrieveCommand,
}

/// # Summary
///
/// The subcommand of the [`Retrieve`] command. Either `employee`, `job`, `location`,
/// `organization`, `person`.
pub enum RetrieveCommand
{
	/// # Summary
	///
	/// The `clinvoice retrieve employee` subcommand.
	Employee
	{
	},

	/// # Summary
	///
	/// The `clinvoice retrieve job` subcommand.
	Job
	{
	},

	/// # Summary
	///
	/// The `clinvoice retrieve location` subcommand.
	Location
	{
	},

	/// # Summary
	///
	/// The `clinvoice retrieve organization` subcommand.
	Organization
	{
	},

	/// # Summary
	///
	/// The `clinvoice retrieve person` subcommand.
	Person
	{
	},
}
