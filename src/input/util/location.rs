use
{
	core::fmt::Display,

	crate::{app::QUERY_PROMPT, DynResult, input},

	clinvoice_adapter::
	{
		data::{Error as DataError, LocationAdapter},
		Store,
	},
	clinvoice_data::views::LocationView,
	clinvoice_query as query,
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
pub fn retrieve_views<'err, D, L>(prompt: D, store: &Store) -> DynResult<'err, Vec<LocationView>> where
	D : Display,
	L : LocationAdapter,

	<L as LocationAdapter>::Error : 'err,
{
	let query: query::Location = input::edit_default(format!("{}\n{}locations", prompt, QUERY_PROMPT))?;

	let results = L::retrieve(&query, &store)?;
	results.into_iter().map(|l|
		L::into_view(l, &store)
	).filter_map(|result| match result
	{
		Ok(t) => match query.matches_view(&t)
		{
			Ok(b) if b => Some(Ok(t)),
			Err(e) => Some(Err(DataError::from(e).into())),
			_ => None,
		},
		Err(e) => Some(Err(e)),
	}).collect::<Result<Vec<_>, _>>().map_err(|e| e.into())
}
