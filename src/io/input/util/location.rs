use
{
	crate::{DynResult, io::input},
	clinvoice_adapter::
	{
		Adapters, Error as AdapterError,
		data::{Error as DataError, LocationAdapter, MatchWhen},
		Store,
	},
	clinvoice_data::views::LocationView,
};

#[cfg(feature="bincode")]
use clinvoice_adapter_bincode::data::{BincodeLocation, Result as BincodeResult};

/// # Summary
///
/// Retrieve all [`Location`][location]s from the specified `store`. If no
/// [`Location`][location]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][L_retrieve] operation fails, its error is forwarded.
/// * If no [`Location`][location]s are [retrieved][L_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [L_retrieve]: clinvoice_adapter::data::LocationAdapter::retrieve
/// [location]: clinvoice_data::Location
pub(super) fn retrieve_or_err<'store, L>(store: &'store Store) -> DynResult<'store, Vec<LocationView>> where
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

/// # Summary
///
/// `prompt` the user to [select](input::select) one [`Location`][location] from the specified `store`.
///
/// # Errors
///
/// * If [`retrieve_or_err`] fails.
/// * If [`input::select_one`] fails.
///
/// [location]: clinvoice_data::Location
pub fn select_one<'store, L, S>(prompt: S, store: &'store Store) -> DynResult<'store, LocationView> where
	L : LocationAdapter<'store> + 'store,
	S : Into<String>,
{
	input::select_one(&retrieve_or_err::<L>(store)?, prompt).map_err(|e| e.into())
}
