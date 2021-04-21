use
{
	std::{borrow::Cow, collections::HashMap, hash::Hash},

	super::{Error, LocationAdapter, query},
	crate::Store,

	clinvoice_data::{Contact, views::ContactView},
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
		Contact::Address {location, export} => match L::retrieve(
			&query::Location
			{
				id: query::Match::EqualTo(Cow::Borrowed(&location)),
				..Default::default()
			},
			store,
		)?.into_iter().next()
		{
			Some(result) => ContactView::Address {location: L::into_view(result, store)?, export},
			_ => return Err(Error::DataIntegrity(location).into()),
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
