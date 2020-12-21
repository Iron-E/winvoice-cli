use crate::Id;

/// # Summary
///
/// A `Client` is an [`Organization`](super::organization::Organization) which has contracted some
/// [`Employer`](super::emlpoyee::Emlpoyer) to do [`Job`](super::job::Job)s.
pub struct Client
{
	/// # Summary
	///
	/// The reference number of the [`Organization`](super::organization::Organization) which this
	/// [`Client`] is in reference to.
	pub organization_id: Id,
}
