use core::fmt::Display;

use clinvoice_adapter::{data::LocationAdapter, Store};
use clinvoice_data::views::LocationView;
use clinvoice_query as query;
use futures::stream::{self, TryStreamExt};

use super::menu;
use crate::{app::QUERY_PROMPT, filter_map_view, input, DynResult};

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
pub async fn retrieve_views<'err, D, L>(
	prompt: D,
	retry_on_empty: bool,
	store: &Store,
) -> DynResult<'err, Vec<LocationView>>
where
	D: Display,
	L: LocationAdapter + Send,

	<L as LocationAdapter>::Error: 'err,
{
	loop
	{
		let query: query::Location =
			input::edit_default(format!("{}\n{}locations", prompt, QUERY_PROMPT))?;

		let results = L::retrieve(&query, &store).await?;
		let results_view: Result<Vec<_>, _> = stream::iter(results.into_iter().map(Ok))
			.map_ok(|l| async move { L::into_view(l, &store).await })
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
