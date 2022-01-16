use core::fmt::Display;

use clinvoice_adapter::{schema::JobAdapter, Deletable};
use clinvoice_match::MatchJob;
use clinvoice_schema::views::JobView;
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
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
/// [L_retrieve]: clinvoice_adapter::schema::LocationAdapter::retrieve
/// [location]: clinvoice_schema::Location
pub async fn retrieve_view<'err, D, Db, JAdapter>(
	connection: &Pool<Db>,
	prompt: D,
	retry_on_empty: bool,
) -> DynResult<'err, Vec<JobView>>
where
	D: Display,
	Db: Database,
	JAdapter: Deletable<Db = Db> + JobAdapter + Send,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	loop
	{
		let match_condition: MatchJob = input::edit_default(format!("{prompt}\n{MATCH_PROMPT}jobs"))?;

		let results = JAdapter::retrieve_view(connection, &match_condition).await?;

		if retry_on_empty && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}
