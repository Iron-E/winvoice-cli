/// # Summary
///
/// The `EmployerConfig` contains information related to the entity which employs the user of the
/// program.
///
/// # Remarks
///
/// The words 'company' and 'organization' are used but should one be employing themselves, the
/// fields of [`EmployerConfig`] are still fitting.
///
/// While it is possible that a particular job could be created under the order of a number of
/// potential organizaations, it may be useful to set one particular organization which is
/// assumed.
///
/// If the details of an employer are desired to be changed later, one may simply alter the
/// information in the generated TOML file.
///
/// # Todo
///
/// It should be possible to create a separate employer listing, and reference an identification
/// number here for the same affect.
///
/// This would also allow for quicker switching of employers on tickets, using the proposed syntax:
///
/// ```sh
/// clinvoice job <job_id> -e <employer_id>
/// ```
pub struct EmployerConfig<'address, 'email, 'name>
{
	/// # Summary
	///
	/// The address of the building where the employer has hired the user from.
	///
	/// # Example
	///
	/// ```rust
	/// EmployerConfig {address: "159 Rusty Road, Town ZipCode"}
	/// ```
	pub address: &'address str,

	/// # Summary
	///
	/// The email address at which a representative of the company can be reached.
	///
	/// # Example
	///
	/// ```rust
	/// EmployerConfig {email: "foo@gmail.com"}
	/// ```
	pub email: &'email str,

	/// # Summary
	///
	/// The name of the company which is the employer.
	///
	/// # Example
	///
	/// ```rust
	/// EmployerConfig {name: "GitHub"}
	/// ```
	pub name: &'name str,
}
