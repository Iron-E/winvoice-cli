use core::fmt::Display;

use clinvoice_adapter::data::{Deletable, OrganizationAdapter};
use clinvoice_data::views::OrganizationView;
use clinvoice_query as query;
use sqlx::{Database, Executor, Pool};

use super::{menu, QUERY_PROMPT};
use crate::{input, DynResult};

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
pub async fn retrieve_view<'err, D, Db, OAdapter>(
	connection: &Pool<Db>,
	prompt: D,
	retry_on_empty: bool,
) -> DynResult<'err, Vec<OrganizationView>>
where
	D: Display,
	Db: Database,
	OAdapter: Deletable<Db = Db> + OrganizationAdapter + Send,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	loop
	{
		let query: query::Organization =
			input::edit_default(format!("{}\n{}organizations", prompt, QUERY_PROMPT))?;

		let results = OAdapter::retrieve_view(connection, &query).await?;

		if retry_on_empty && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}
