use clinvoice_data::uuid::Uuid;

/// # Summary
///
/// Configurations for [`Employee`](clinvoice_data::employee::Employee)s.
#[derive(Debug)]
pub struct Employees
{
	/// # Summary
	///
	/// The [`Uuid`] of the employee which should be defaulted to when attaching to a timesheet.
	pub default_id: Uuid,
}
