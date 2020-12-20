use super::id::Id;

/// # Summary
///
/// An `Organization` is a facilitator of business.
///
/// # Remarks
///
/// An `Organization` can be a person, or an entire business. If one is self-employed, then the
/// `Organization` would simply be themselves.
///
/// An `Organization` has no specific affitilation to the user, and as such can be both a
/// [`Client`](super::client::Client) _and_ an [`Emlpoyer`](super::employer::Employer) at the same
/// time.
pub struct Organization<'name>
{
	/// # Summary
	///
	/// The unique reference number for this [`Organization`].
	_id: Id,

	/// # Summary
	///
	/// The reference umber of the [`Location`](super::location::Location) where this
	/// [`Organization`] resides.
	_location_id: Id,

	/// # Summary
	///
	/// The name of the [`Organization`].
	pub name: &'name str,
}
