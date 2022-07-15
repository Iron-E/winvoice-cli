use core::fmt::Display;

use clinvoice_adapter::{schema::LocationAdapter, Deletable};
use clinvoice_match::MatchLocation;
use clinvoice_schema::Location;
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
use crate::{input, DynResult};

/// Retrieve all [`Location`][location]s from the specified `store` that match a user-provided
/// query. If `RETRY_ON_EMPTY`, the query is attempted again when the query returns no results.
pub async fn retrieve<LAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<Location>>
where
	LAdapter: Deletable<Db = TDb> + LocationAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	loop
	{
		let match_condition: MatchLocation =
			input::edit_default(format!("{prompt}\n{MATCH_PROMPT}locations"))?;

		let results = LAdapter::retrieve(connection, &match_condition).await?;

		if RETRY_ON_EMPTY && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}

/// [Retrieve](retrieve) `Location`s and then [select one](input::select_one) of them.
pub async fn select_one<LAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Location>
where
	LAdapter: Deletable<Db = TDb> + LocationAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let locations = retrieve::<LAdapter, TDb, TPrompt, RETRY_ON_EMPTY>(connection, prompt).await?;
	let location = input::select_one(&locations, "Select the `Location`")?;

	Ok(location)
}
