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
	std::{borrow::Cow, collections::HashMap, hash::Hash},
};

/// # Summary
///
/// Convert some [`Contact`] into a [`ContactView`].
pub fn into_view(contact: Contact, store: &Store) -> Result<ContactView>
{
	Ok(match contact
	{
		Contact::Address(address) => match BincodeLocation::retrieve(
				MatchWhen::EqualTo(Cow::Borrowed(&address)), // id
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
pub fn into_views<I, T>(contact_info: I, store: &Store) -> Result<HashMap<T, ContactView>> where
	I : IntoIterator<Item=(T, Contact)>,
	T : Eq + Hash,
{
	contact_info.into_iter().map(|(key, contact)|
		into_view(contact, store).map(|view| (key, view))
	).collect()
}
