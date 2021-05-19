use
{
	std::{borrow::Cow::Borrowed, collections::HashMap, hash::Hash},

	super::{Error, LocationAdapter},
	crate::Store,

	clinvoice_data::{Contact, views::ContactView},
	clinvoice_query as query,
};

/// # Summary
///
/// Convert some [`Contact`] into a [`ContactView`].
pub fn to_view<L>(contact: Contact, store: &Store)
	-> Result<ContactView, <L as LocationAdapter>::Error>
where
	L : LocationAdapter
{
	Ok(match contact
	{
		Contact::Address {location_id, export} =>
		{
			let results = L::retrieve(
				&query::Location
				{
					id: query::Match::EqualTo(Borrowed(&location_id)),
					..Default::default()
				},
				store,
			)?;

			let location = results.into_iter().next().ok_or_else(|| Error::DataIntegrity(location_id))?;

			ContactView::Address
			{
				location: L::into_view(location, store)?,
				export,
			}
		},

		Contact::Email {email, export} => ContactView::Email {email, export},
		Contact::Phone {phone, export} => ContactView::Phone {phone, export},
	})
}

/// # Summary
///
/// Convert some [`Contact`] into a [`ContactView`].
pub fn to_views<L, T>(contact_info: HashMap<T, Contact>, store: &Store)
	-> Result<HashMap<T, ContactView>, <L as LocationAdapter>::Error>
where
	L : LocationAdapter,
	T : Eq + Hash,
{
	contact_info.into_iter().map(|(key, contact)|
		to_view::<L>(contact, store).map(|view| (key, view))
	).collect()
}
