use core::fmt::Display;

use clinvoice_adapter::{
	data::{LocationAdapter, OrganizationAdapter},
	Store,
};
use clinvoice_data::views::OrganizationView;
use clinvoice_query as query;
use futures::stream::{self, TryStreamExt};

use super::menu;
use crate::{app::QUERY_PROMPT, filter_map_view, input, DynResult};

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
pub async fn retrieve_views<'err, D, L, O>(
	prompt: D,
	retry_on_empty: bool,
	store: &Store,
) -> DynResult<'err, Vec<OrganizationView>>
where
	D: Display,
	L: LocationAdapter + Send,
	O: OrganizationAdapter + Send,

	<L as LocationAdapter>::Error: 'err,
	<O as OrganizationAdapter>::Error: 'err,
{
	loop
	{
		let query: query::Organization =
			input::edit_default(format!("{}\n{}organizations", prompt, QUERY_PROMPT))?;

		let results = O::retrieve(&query, &store).await?;
		let results_view: Result<Vec<_>, _> = stream::iter(results.into_iter().map(Ok))
			.map_ok(|o| async move { O::into_view::<L>(o, &store).await })
			.try_buffer_unordered(10)
			.try_filter_map(|val| filter_map_view!(query, val))
			.try_collect()
			.await;

		if retry_on_empty &&
			results_view.as_ref().map(Vec::is_empty).unwrap_or(false) &&
			menu::retry_query()?
		{
			continue;
		}

		return results_view.map_err(|e| e.into());
	}
}
