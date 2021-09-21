use core::fmt::Display;

use clinvoice_adapter::data::{Deletable, LocationAdapter};
use clinvoice_data::views::LocationView;
use clinvoice_query as query;
use sqlx::{Database, Executor, Pool};

use super::{menu, QUERY_PROMPT};
use crate::{input, DynResult};

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
pub async fn retrieve_view<'err, D, Db, LAdapter>(
	connection: &Pool<Db>,
	prompt: D,
	retry_on_empty: bool,
) -> DynResult<'err, Vec<LocationView>>
where
	D: Display,
	Db: Database,
	LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
	<LAdapter as Deletable>::Error: 'err,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	loop
	{
		let query: query::Location =
			input::edit_default(format!("{}\n{}locations", prompt, QUERY_PROMPT))?;

		let results = LAdapter::retrieve_view(connection, &query).await?;

		if retry_on_empty && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}
