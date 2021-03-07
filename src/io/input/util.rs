use
{
	crate::DynResult,
	clinvoice_adapter::
	{
		Adapters, Error as AdapterError,
		data::{Error as DataError, LocationAdapter, MatchWhen},
		Store,
	},
	clinvoice_data::
	{
		Contact,
		views::{ContactView, LocationView}
	},
	serde::{Deserialize, Serialize},
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodeLocation, Result as BincodeResult};

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
fn retrieve_locations_or_err<'store, L>(store: &'store Store) -> DynResult<'store, Vec<LocationView>> where
	L : LocationAdapter<'store> + 'store,
{
	let locations = L::retrieve(MatchWhen::Any, MatchWhen::Any, MatchWhen::Any, store)?;

	if locations.is_empty()
	{
		return Err(DataError::NoData {entity: stringify!(Location)}.into());
	}

	locations.into_iter().try_fold(Vec::new(),
		|mut v, l| -> DynResult<Vec<LocationView>>
		{
			v.push(match store.adapter
			{
				#[cfg(feature="bincode")]
				Adapters::Bincode =>
				{
					let result: BincodeResult<LocationView> = BincodeLocation {location: &l, store}.into();
					result
				},

				_ => return Err(AdapterError::FeatureNotFound {adapter: store.adapter}.into()),
			}?);

			Ok(v)
		},
	)
}

pub fn select_contact_info<'store, L>(store: &'store Store) -> DynResult<'store, Vec<Contact>> where
	L : LocationAdapter<'store> + 'store,
{
	let locations = retrieve_locations_or_err::<L>(store)?;

	let mut contact_info = super::select(
		&locations.into_iter().map(|l| l.into()).collect::<Vec<ContactView>>(),
		"Select locations to be a part of the contact info.",
	)?;

	contact_info.push(ContactView::Email("An email address. E.g. `foo@gmail.com`".into()));
	contact_info.push(ContactView::Phone("A phone number. E.g. `600-555-5555`".into()));

	Ok(super::edit(SerdeWrapper {value: contact_info})?.value.into_iter().map(|c| c.into()).collect())
}

pub fn select_one_location<'store, L, S>(prompt: S, store: &'store Store) -> DynResult<'store, LocationView> where
	L : LocationAdapter<'store> + 'store,
	S : Into<String>,
{
	super::select_one(
		&retrieve_locations_or_err::<L>(store)?,
		prompt,
	).map_err(|e| e.into())
}
