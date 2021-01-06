use clinvoice_data::Employee;

mod into_organization;
mod into_person;

/// # Summary
///
/// Wrapper around [`Employee`] for use with TomlDB.
pub struct TomlEmployee<'contact_info, 'email, 'phone>
(
	Employee<'contact_info, 'email, 'phone>,
);
