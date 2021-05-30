use
{
	core::fmt::Display,

	super::menu,
	crate::{app::QUERY_PROMPT, DynResult, filter_map_view, input},

	clinvoice_adapter::
	{
		data::{Error as DataError, EmployeeAdapter, JobAdapter, LocationAdapter, OrganizationAdapter, PersonAdapter},
		Store,
	},
	clinvoice_data::views::JobView,
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
pub fn retrieve_views<'err, D, E, J, L, O, P>(prompt: D, retry_on_empty: bool, store: &Store) -> DynResult<'err, Vec<JobView>> where
	D : Display,
	E : EmployeeAdapter,
	J : JobAdapter,
	L : LocationAdapter,
	O : OrganizationAdapter,
	P : PersonAdapter,

	<E as EmployeeAdapter>::Error :
		From<<L as LocationAdapter>::Error> +
		From<<O as OrganizationAdapter>::Error> +
		From<<P as PersonAdapter>::Error>,
	<J as JobAdapter>::Error : 'err + From<<E as EmployeeAdapter>::Error>,
{
	let query: query::Job = input::edit_default(format!("{}\n{}jobs", prompt, QUERY_PROMPT))?;

	let results = J::retrieve(&query, &store)?;
	let results_view: Result<Vec<_>, _> =results.into_iter().map(|j|
		J::into_view::<E, L, O, P>(j, &store)
	).filter_map(|result| filter_map_view!(query, result)).collect();

	if retry_on_empty && results_view.as_ref().map(|r| r.is_empty()).unwrap_or(false) && menu::retry_query()?
	{
		return retrieve_views::<D, E, J, L, O, P>(prompt, true, store);
	}

	results_view.map_err(|e| e.into())
}
