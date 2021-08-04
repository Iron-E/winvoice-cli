use core::fmt::Display;

use clinvoice_adapter::{
	data::{
		Error as DataError,
		LocationAdapter,
		OrganizationAdapter,
	},
	Store,
};
use clinvoice_data::views::OrganizationView;
use clinvoice_query as query;

use super::menu;
use crate::{
	app::QUERY_PROMPT,
	filter_map_view,
	input,
	DynResult,
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
pub fn retrieve_views<'err, D, L, O>(
	prompt: D,
	retry_on_empty: bool,
	store: &Store,
) -> DynResult<'err, Vec<OrganizationView>>
where
	D: Display,
	L: LocationAdapter,
	O: OrganizationAdapter,

	<L as LocationAdapter>::Error: 'err,
	<O as OrganizationAdapter>::Error: 'err,
{
	let query: query::Organization =
		input::edit_default(format!("{}\n{}organizations", prompt, QUERY_PROMPT))?;

	let results = O::retrieve(&query, &store)?;
	let results_view: Result<Vec<_>, _> = results
		.into_iter()
		.map(|o| O::into_view::<L>(o, &store))
		.filter_map(|result| filter_map_view!(query, result))
		.collect();

	if retry_on_empty &&
		results_view.as_ref().map(|r| r.is_empty()).unwrap_or(false) &&
		menu::retry_query()?
	{
		return retrieve_views::<D, L, O>(prompt, true, store);
	}

	results_view.map_err(|e| e.into())
}
