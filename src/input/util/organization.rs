use
{
	crate::{app::QUERY_PROMPT, DynResult, input},

	clinvoice_adapter::
	{
		data::{Error as DataError, LocationAdapter, OrganizationAdapter},
		Store,
	},
	clinvoice_data::views::OrganizationView,
	clinvoice_query as query,
};

/// # Summary
///
/// Retrieve all [`Organization`][organization]s from the specified `store`. If no
/// [`Organization`][organization]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][P_retrieve] operation fails, its error is forwarded.
/// * If no [`Organization`][organization]s are [retrieved][P_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [P_retrieve]: clinvoice_adapter::data::OrganizationAdapter::retrieve
/// [organization]: clinvoice_data::Organization
pub fn retrieve_views<'err, L, O>(store: &Store) -> DynResult<'err, Vec<OrganizationView>> where
	L : LocationAdapter,
	O : OrganizationAdapter,

	<L as LocationAdapter>::Error : 'err,
	<O as OrganizationAdapter>::Error : 'err,
{
	let query: query::Organization = input::edit_default(format!("{}organizations", QUERY_PROMPT))?;

	let results = O::retrieve(&query, &store)?;
	results.into_iter().map(|o| O::into_view::<L>(o, &store)).filter_map(|result| match result
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
