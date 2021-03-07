use
{
	super::BincodeLocation,
	crate::data::Result,
	clinvoice_adapter::
	{
		data::{Error as DataError, LocationAdapter, MatchWhen},
		Store,
	},
	clinvoice_data::{Contact, views::{ContactView, LocationView}},
};

/// # Summary
///
/// Convert some [`Contact`] into a [`ContactView`].
pub fn into_view(contact: Contact, store: &Store) -> Result<ContactView>
{
	Ok(match contact
	{
		Contact::Address(address) => match BincodeLocation::retrieve(
				MatchWhen::EqualTo(address), // id
				MatchWhen::Any, // outer_id
				MatchWhen::Any, // name
				store,
			)?.into_iter().next()
			{
				Some(result) =>
				{
					let view: Result<LocationView> = BincodeLocation {location: &result, store}.into();
					view?.into()
				},
				_ => return Err(DataError::DataIntegrity {id: address}.into()),
			},
		Contact::Email(email) => ContactView::Email(email),
		Contact::Phone(phone) => ContactView::Phone(phone),
	})
}

/// # Summary
///
/// Convert some [`Contact`] into a [`ContactView`].
pub fn into_views<I>(contact_info: I, store: &Store) -> Result<Vec<ContactView>> where
	I : IntoIterator<Item=Contact>,
{
	contact_info.into_iter().map(|c| into_view(c, store)).collect()
}
