use std::{borrow::Cow::Borrowed, collections::HashMap, hash::Hash, marker::Send};

use clinvoice_data::{views::ContactView, Contact};
use clinvoice_query as query;
use futures::{
	stream::{self, StreamExt, TryStreamExt},
	FutureExt,
	TryFutureExt,
};

use super::{Error, LocationAdapter};
use crate::Store;

/// # Summary
///
/// Convert some [`Contact`] into a [`ContactView`].
pub async fn into_view<L>(
	contact: Contact,
	store: &Store,
) -> Result<ContactView, <L as LocationAdapter>::Error>
where
	L: LocationAdapter + Send,
{
	Ok(match contact
	{
		Contact::Address {
			location_id,
			export,
		} =>
		{
			let query = query::Location {
				id: query::Match::EqualTo(Borrowed(&location_id)),
				..Default::default()
			};

			let location = L::retrieve(&query, store)
				.map(|result| {
					result.and_then(|retrieved| {
						retrieved
							.into_iter()
							.next()
							.ok_or(Error::DataIntegrity(location_id))
							.map_err(|e| e.into())
					})
				})
				.and_then(|location| L::into_view(location, store));

			ContactView::Address {
				location: location.await?,
				export,
			}
		},

		Contact::Email { email, export } => ContactView::Email { email, export },
		Contact::Phone { phone, export } => ContactView::Phone { phone, export },
	})
}

/// # Summary
///
/// Convert some [`Contact`] into a [`ContactView`].
pub async fn into_views<L, T>(
	contact_info: HashMap<T, Contact>,
	store: &Store,
) -> Result<HashMap<T, ContactView>, <L as LocationAdapter>::Error>
where
	L: LocationAdapter + Send,
	T: Eq + Hash,
{
	stream::iter(contact_info.into_iter())
		.then(|(key, contact)| async { into_view::<L>(contact, store).await.map(|view| (key, view)) })
		.try_collect()
		.await
}
