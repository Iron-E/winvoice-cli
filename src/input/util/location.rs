use
{
	crate::{DynResult, input},

	clinvoice_adapter::
	{
		data::{Error as DataError, LocationAdapter},
		Store,
	},
	clinvoice_data::views::LocationView,
};

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
pub(super) fn retrieve_or_err<'err, L>(store: &Store) -> DynResult<'err, Vec<LocationView>> where
	L : LocationAdapter,
	<L as LocationAdapter>::Error : 'err,
{
	let locations = L::retrieve(Default::default(), store)?;

	if locations.is_empty()
	{
		return Err(DataError::NoData(stringify!(Location)).into());
	}

	let locations_len = locations.len();
	locations.into_iter().try_fold(
		Vec::with_capacity(locations_len),
		|mut v, l| -> DynResult<'err, Vec<LocationView>>
		{
			v.push(L::into_view(l, store)?);

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
pub fn select_one<'err, L, S>(prompt: S, store: &Store) -> DynResult<'err, LocationView> where
	L : LocationAdapter,
	<L as LocationAdapter>::Error : 'err,
	S : Into<String>,
{
	let retrieved = retrieve_or_err::<L>(store)?;
	input::select_one(&retrieved, prompt).map_err(|e| e.into())
}
