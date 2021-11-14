use core::fmt::Display;

use clinvoice_adapter::{schema::PersonAdapter, Deletable};
use clinvoice_match::MatchPerson;
use clinvoice_schema::views::PersonView;
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
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
/// [P_retrieve]: clinvoice_adapter::schema::PersonAdapter::retrieve
/// [person]: clinvoice_schema::Person
pub async fn retrieve_view<'err, D, Db, PAdapter>(
	connection: &Pool<Db>,
	prompt: D,
	retry_on_empty: bool,
) -> DynResult<'err, Vec<PersonView>>
where
	D: Display,
	Db: Database,
	PAdapter: Deletable<Db = Db> + PersonAdapter + Send,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	loop
	{
		let match_condition: MatchPerson =
			input::edit_default(format!("{}\n{}persons", prompt, MATCH_PROMPT))?;

		let results = PAdapter::retrieve_view(connection, &match_condition).await?;

		if retry_on_empty && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}
