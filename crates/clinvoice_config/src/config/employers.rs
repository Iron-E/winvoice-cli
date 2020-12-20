use clinvoice_data::id::Id;

/// # Summary
///
/// Configurations for [`Employer`](clinvoice_data::employee::Employer)s.
pub struct Employers
{
	/// # Summary
	///
	/// The [`Id`] of the employer which should be defaulted to when creating a new
	/// [`Employee`](clinvoice_data::employee::Employee).
	pub default_id: Id,
}
