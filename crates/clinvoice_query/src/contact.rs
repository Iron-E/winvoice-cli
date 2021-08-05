use clinvoice_data::views::ContactView;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Location, MatchStr, Result};

/// # Summary
///
/// A method through which something can be communicated with.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Contact<'m>
{
	#[cfg_attr(feature = "serde_support", serde(default))]
	pub address: Location<'m>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub email: MatchStr<String>,

	#[cfg_attr(feature = "serde_support", serde(default))]
	pub phone: MatchStr<String>,
}

impl Contact<'_>
{
	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn set_matches<'item>(
		&self,
		contact_info: &mut impl Iterator<Item = &'item clinvoice_data::Contact>,
	) -> Result<bool>
	{
		Ok(self.address.id.set_matches(
			&contact_info
				.by_ref()
				.flat_map(|c| match c
				{
					clinvoice_data::Contact::Address {
						location_id,
						export: _,
					} => Some(location_id),
					_ => None,
				})
				.collect(),
		) && self
			.email
			.set_matches(&mut contact_info.by_ref().flat_map(|c| match c
			{
				clinvoice_data::Contact::Email { email, export: _ } => Some(email.as_ref()),
				_ => None,
			}))? && self
			.phone
			.set_matches(&mut contact_info.flat_map(|c| match c
			{
				clinvoice_data::Contact::Phone { phone, export: _ } => Some(phone.as_ref()),
				_ => None,
			}))?)
	}

	/// # Summary
	///
	/// Return `true` if `employee` is a match.
	pub fn set_matches_view<'item>(
		&self,
		contact_info: &mut impl Iterator<Item = &'item ContactView>,
	) -> Result<bool>
	{
		Ok(self
			.address
			.set_matches_view(&mut contact_info.by_ref().flat_map(|c| match c
			{
				ContactView::Address {
					location,
					export: _,
				} => Some(location),
				_ => None,
			}))? && self
			.email
			.set_matches(&mut contact_info.by_ref().flat_map(|c| match c
			{
				ContactView::Email { email, export: _ } => Some(email.as_ref()),
				_ => None,
			}))? && self
			.phone
			.set_matches(&mut contact_info.flat_map(|c| match c
			{
				ContactView::Phone { phone, export: _ } => Some(phone.as_ref()),
				_ => None,
			}))?)
	}
}
