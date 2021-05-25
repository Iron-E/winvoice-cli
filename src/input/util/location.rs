use
{
	core::fmt::Display,

	crate::{app::QUERY_PROMPT, DynResult, filter_map_view, input},

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
pub fn retrieve_views<'err, D, L>(prompt: D, retry_on_empty: bool, store: &Store) -> DynResult<'err, Vec<LocationView>> where
	D : Display,
	L : LocationAdapter,

	<L as LocationAdapter>::Error : 'err,
{
	let query: query::Location = input::edit_default(format!("{}\n{}locations", prompt, QUERY_PROMPT))?;

	let results = L::retrieve(&query, &store)?;
	let results_view: Result<Vec<_>, _> =results.into_iter().map(|l|
		L::into_view(l, &store)
	).filter_map(|result| filter_map_view!(query, result)).collect();

	if retry_on_empty && results_view.as_ref().map(|r| r.is_empty()).unwrap_or(false)
	{
		todo!("raise retry menu");
	}

	results_view.map_err(|e| e.into())
}
