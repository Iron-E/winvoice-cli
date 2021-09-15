use core::fmt::Display;

use clinvoice_adapter::data::OrganizationAdapter;
use clinvoice_data::views::OrganizationView;
use clinvoice_query as query;

use super::menu;
use crate::{app::QUERY_PROMPT, input, DynResult};

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
pub async fn retrieve_view<'a, D, O, P>(
	prompt: D,
	retry_on_empty: bool,
	pool: &'a P,
) -> DynResult<'a, Vec<OrganizationView>>
where
	D: Display,
	O: OrganizationAdapter<Pool = &'a P> + Send,
	<O as OrganizationAdapter>::Error: 'a,
{
	loop
	{
		let query: query::Organization =
			input::edit_default(format!("{}\n{}organizations", prompt, QUERY_PROMPT))?;

		let results = O::retrieve_view(&query, pool).await?;

		if retry_on_empty &&
			results.is_empty() &&
			menu::retry_query()?
		{
			continue;
		}

		return Ok(results);
	}
}
