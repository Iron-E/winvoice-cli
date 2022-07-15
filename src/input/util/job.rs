use core::fmt::Display;

use clinvoice_adapter::{schema::JobAdapter, Deletable};
use clinvoice_match::MatchJob;
use clinvoice_schema::Job;
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
use crate::{input, DynResult};

/// Retrieve all [`Job`][location]s from the specified `store` that match a user-provided
/// query. If `RETRY_ON_EMPTY`, the query is attempted again when the query returns no results.
pub async fn retrieve<JAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<Job>>
where
	JAdapter: Deletable<Db = TDb> + JobAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	loop
	{
		let match_condition: MatchJob =
			input::edit_default(format!("{prompt}\n{MATCH_PROMPT}locations"))?;

		let results = JAdapter::retrieve(connection, &match_condition).await?;

		if RETRY_ON_EMPTY && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}

/// [Retrieve](retrieve) `Job`s and then [select](input::select) from them.
pub async fn select<JAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<Job>>
where
	JAdapter: Deletable<Db = TDb> + JobAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let locations = retrieve::<JAdapter, TDb, TPrompt, RETRY_ON_EMPTY>(connection, prompt).await?;
	let selected = input::select(&locations, "Select the `Job`s")?;

	Ok(selected)
}

/// [Retrieve](retrieve) `Job`s and then [select one](input::select_one) of them.
pub async fn select_one<JAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Job>
where
	JAdapter: Deletable<Db = TDb> + JobAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let locations = retrieve::<JAdapter, TDb, TPrompt, RETRY_ON_EMPTY>(connection, prompt).await?;
	let selected = input::select_one(&locations, "Select the `Job`")?;

	Ok(selected)
}
