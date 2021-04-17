use
{
	std::{borrow::Cow, collections::HashMap, hash::Hash},

	super::{Error, LocationAdapter, Match, query},
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
		Contact::Address(address) => match L::retrieve(
			&query::Location
			{
				id: Match::EqualTo(Cow::Borrowed(&address)),
				..Default::default()
			},
			store,
		)?.into_iter().next()
		{
			Some(result) => L::into_view(result, store)?.into(),
			_ => return Err(Error::DataIntegrity(address).into()),
		},
		Contact::Email(email) => ContactView::Email(email),
		Contact::Phone(phone) => ContactView::Phone(phone),
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
