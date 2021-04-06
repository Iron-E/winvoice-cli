use
{
	crate::io::input,

	clinvoice_data::EmployeeStatus,

	std::io::Result,
};

/// # Summary
///
/// `prompt` the user to [select](input::select) one [`Location`][organization] from the specified `store`.
///
/// # Errors
///
/// * If [`retrieve_or_err`] fails.
/// * If [`input::select_one`] fails.
///
/// [organization]: clinvoice_data::Organization
pub fn select_one(prompt: impl Into<String>) -> Result<EmployeeStatus>
{
	input::select_one(
		&[
			EmployeeStatus::Employed,
			EmployeeStatus::NotEmployed,
			EmployeeStatus::Representative,
		],
		prompt,
	)
}
