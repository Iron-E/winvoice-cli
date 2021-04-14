use
{
	crate::DynResult,

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
pub fn retrieve_views<'err, L>(store: &Store) -> DynResult<'err, Vec<LocationView>> where
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
