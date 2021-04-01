use
{
	super::Location,
	crate::data::Match,
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
	pub address: Location<'m>,
	pub email: Match<'m, String>,
	pub phone: Match<'m, String>,
}

impl Contact<'_>
{
	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn any_matches_view<'item>(&self, contact_info: impl Clone + Iterator<Item=&'item ContactView>) -> bool
	{
		contact_info.clone().flat_map(|c| match c
		{
			ContactView::Address(a) => Some(a),
			_ => None,
		}).any(|a| self.address.matches_view(a)) &&
		self.email.set_matches(
			&contact_info.clone().flat_map(|c| match c
			{
				ContactView::Email(e) => Some(e),
				_ => None,
			}).collect()
		) && self.phone.set_matches(
			&contact_info.flat_map(|c| match c
			{
				ContactView::Email(p) => Some(p),
				_ => None,
			}).collect()
		)
	}

	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn set_matches<'item>(&self, contact_info: impl Clone + Iterator<Item=&'item clinvoice_data::Contact>) -> bool
	{
		self.address.id.set_matches(
			&contact_info.clone().flat_map(|c| match c
			{
				clinvoice_data::Contact::Address(a) => Some(a),
				_ => None,
			}).collect()
		) && self.email.set_matches(
			&contact_info.clone().flat_map(|c| match c
			{
				clinvoice_data::Contact::Email(e) => Some(e),
				_ => None,
			}).collect()
		) && self.phone.set_matches(
			&contact_info.flat_map(|c| match c
			{
				clinvoice_data::Contact::Phone(p) => Some(p),
				_ => None,
			}).collect()
		)
	}
}
