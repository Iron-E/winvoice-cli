use
{
	crate::DynResult,
	clinvoice_adapter::
	{
		data::{Error, LocationAdapter, MatchWhen},
		Store,
	},
	clinvoice_data::
	{
		Contact,
		views::{ContactView, LocationView}
	},
	serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize)]
struct SerdeWrapper<T> { value: T }

/// # Summary
///
/// Retrieve all [`Location`][location]s from the specified `store` and [select](super::select)
/// them with a `prompt`. If not [`Location`][location]s are retrieved, return an
/// [error](Error::NoData).
///
/// # Errors
///
/// * If the [retrieval][L_retrieve] operation fails, its error is forwarded.
/// * If no [`Location`][location]s are [retrieved][L_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](super::select) operation fails, its error is forwarded.
///
/// [L_retrieve]: clinvoice_adapter::data::LocationAdapter::retrieve
/// [location]: clinvoice_data::Location
fn retrieve_locations_or_err<'store, L>(store: &Store) -> DynResult<Vec<LocationView>> where
	L : LocationAdapter<'store>,
{
	let locations = L::retrieve(MatchWhen::Any, MatchWhen::Any, MatchWhen::Any, store)?;

	if locations.is_empty()
	{
		return Err(Error::NoData {entity: stringify!(Location)}.into());
	}

	Ok(locations.into_iter().try_fold(Vec::new(),
		|mut v, l| -> Result<Vec<LocationView>, <L as LocationAdapter<'store>>::Error>
		{
			let result: Result<LocationView, <L as LocationAdapter<'store>>::Error> = l.into();
			v.push(result?);
			Ok(v)
		},
	)?)
}

pub fn select_contact_info<'store, L>(store: &Store) -> DynResult<Vec<Contact>> where
	L : LocationAdapter<'store>,
{
	let locations = super::select(
		&retrieve_locations_or_err::<L>(store)?,
		"Select locations to be part of the contact info.",
	)?;

	let mut contact_info = super::select(
		&locations.into_iter().map(|l| l.into()).collect::<Vec<ContactView>>(),
		"Select locations to be a part of the contact info.",
	)?;

	contact_info.push(ContactView::Email("An email address. E.g. `foo@gmail.com`".into()));
	contact_info.push(ContactView::Phone("A phone number. E.g. `600-555-5555`".into()));

	Ok(super::edit(SerdeWrapper {value: contact_info})?.value.into_iter().map(|c| c.into()).collect())
}

pub fn select_one_location<'store, L, S>(prompt: S, store: &Store) -> DynResult<LocationView> where
	L : LocationAdapter<'store>,
	S : Into<String>,
{
	super::select_one(
		&retrieve_locations_or_err::<L>(store)?,
		prompt,
	).map_err(|e| e.into())
}
