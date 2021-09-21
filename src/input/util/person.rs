use core::fmt::Display;

use clinvoice_adapter::data::{Deletable, PersonAdapter};
use clinvoice_data::views::PersonView;
use clinvoice_query as query;
use sqlx::{Database, Executor, Pool};

use super::{menu, QUERY_PROMPT};
use crate::{input, DynResult};

/// # Summary
///
/// Retrieve all [`Person`][person]s from the specified `store`. If no
/// [`Person`][person]s are retrieved, return an [error](DataError::NoData).
///
/// # Errors
///
/// * If the [retrieval][P_retrieve] operation fails, its error is forwarded.
/// * If no [`Person`][person]s are [retrieved][P_retrieve], an [`Error::NoData`] is returned.
/// * If the [selection](input::select) operation fails, its error is forwarded.
///
/// [P_retrieve]: clinvoice_adapter::data::PersonAdapter::retrieve
/// [person]: clinvoice_data::Person
pub async fn retrieve_view<'err, D, Db, PAdapter>(
	connection: &Pool<Db>,
	prompt: D,
	retry_on_empty: bool,
) -> DynResult<'err, Vec<PersonView>>
where
	D: Display,
	Db: Database,
	PAdapter : Deletable<Db = Db> + PersonAdapter + Send,
	<PAdapter as Deletable>::Error: 'err,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	loop
	{
		let query: query::Person =
			input::edit_default(format!("{}\n{}persons", prompt, QUERY_PROMPT))?;

		let results = PAdapter::retrieve_view(connection, &query).await?;

		if retry_on_empty &&
			results.is_empty() &&
			menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}
