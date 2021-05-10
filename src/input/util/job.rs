use
{
	crate::{app::QUERY_PROMPT, DynResult, input},

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
pub fn retrieve_views<'err, E, J, L, O, P>(store: &Store) -> DynResult<'err, Vec<JobView>> where
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
	let query: query::Job = input::edit_default(String::from(QUERY_PROMPT) + "jobs")?;

	let results = J::retrieve(&query, &store)?;
	results.into_iter().map(|j|
		J::into_view::<E, L, O, P>(j, &store)
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
