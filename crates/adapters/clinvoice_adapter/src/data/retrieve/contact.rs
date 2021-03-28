use
{
	super::Location,
	crate::data::MatchWhen,
	clinvoice_data::views::ContactView,
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// A method through which something can be communicated with.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Contact<'m>
{
	/// # Summary
	///
	/// A [`Location`](crate::Location).
	pub address: Location<'m>,

	/// # Summary
	///
	/// An email address.
	///
	/// # Example
	///
	/// * 'foo@bar.io'
	pub email: MatchWhen<'m, String>,

	/// # Summary
	///
	/// A phone number.
	///
	/// # Example
	///
	/// * '1-603-555-1234'
	/// * '603-555-1234'
	pub phone: MatchWhen<'m, String>,
}

impl Contact<'_>
{
	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn any_matches_view(&self, contact_info: &[&ContactView]) -> bool
	{
		self.address.any_matches_view(
			&contact_info.iter().flat_map(|c| match c
			{
				ContactView::Address(a) => Some(a),
				_ => None,
			}).collect::<Vec<_>>()
		) && self.email.set_matches(
			&contact_info.iter().flat_map(|c| match c
			{
				ContactView::Email(e) => Some(e),
				_ => None,
			}).collect()
		) && self.phone.set_matches(
			&contact_info.iter().flat_map(|c| match c
			{
				ContactView::Email(p) => Some(p),
				_ => None,
			}).collect()
		)
	}

	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn set_matches(&self, contact_info: &[&clinvoice_data::Contact]) -> bool
	{
		self.address.id.set_matches(
			&contact_info.iter().flat_map(|c| match c
			{
				clinvoice_data::Contact::Address(a) => Some(a),
				_ => None,
			}).collect()
		) && self.email.set_matches(
			&contact_info.iter().flat_map(|c| match c
			{
				clinvoice_data::Contact::Email(e) => Some(e),
				_ => None,
			}).collect()
		) && self.phone.set_matches(
			&contact_info.iter().flat_map(|c| match c
			{
				clinvoice_data::Contact::Phone(p) => Some(p),
				_ => None,
			}).collect()
		)
	}
}
