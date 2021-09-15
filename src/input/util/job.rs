use core::fmt::Display;

use clinvoice_adapter::data::JobAdapter;

use clinvoice_data::views::JobView;
use clinvoice_query as query;

use super::menu;
use crate::{app::QUERY_PROMPT, input, DynResult};

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
pub async fn retrieve_view<'a, D, J, P>(
	prompt: D,
	retry_on_empty: bool,
	pool: &'a P,
) -> DynResult<'a, Vec<JobView>>
where
	D: Display,
	J: JobAdapter<Pool = &'a P> + Send,
	<J as JobAdapter>::Error: 'a,
{
	loop
	{
		let query: query::Job = input::edit_default(format!("{}\n{}jobs", prompt, QUERY_PROMPT))?;

		let results = J::retrieve_view(&query, pool).await?;

		if retry_on_empty &&
			results.is_empty() &&
			menu::retry_query()?
		{
			continue;
		}

		return Ok(results);
	}
}
