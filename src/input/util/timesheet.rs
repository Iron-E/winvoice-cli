use core::fmt::Display;

use clinvoice_adapter::{schema::TimesheetAdapter, Deletable};
use clinvoice_match::MatchTimesheet;
use clinvoice_schema::Timesheet;
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
use crate::{input, DynResult};

/// Retrieve all [`Timesheet`][location]s from the specified `store` that match a user-provided
/// query. If `RETRY_ON_EMPTY`, the query is attempted again when the query returns no results.
pub async fn retrieve<TAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<Timesheet>>
where
	TAdapter: Deletable<Db = TDb> + TimesheetAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	loop
	{
		let match_condition: MatchTimesheet =
			input::edit_default(format!("{prompt}\n{MATCH_PROMPT}locations"))?;

		let results = TAdapter::retrieve(connection, &match_condition).await?;

		if RETRY_ON_EMPTY && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}

/// [Retrieve](retrieve) `Timesheet`s and then [select](input::select) from them.
pub async fn select<TAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<Timesheet>>
where
	TAdapter: Deletable<Db = TDb> + TimesheetAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let locations = retrieve::<TAdapter, TDb, TPrompt, RETRY_ON_EMPTY>(connection, prompt).await?;
	let selected = input::select(&locations, "Select the `Timesheet`s")?;

	Ok(selected)
}

/// [Retrieve](retrieve) `Timesheet`s and then [select one](input::select_one) of them.
pub async fn select_one<TAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Timesheet>
where
	TAdapter: Deletable<Db = TDb> + TimesheetAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let locations = retrieve::<TAdapter, TDb, TPrompt, RETRY_ON_EMPTY>(connection, prompt).await?;
	let selected = input::select_one(&locations, "Select the `Timesheet`")?;

	Ok(selected)
}
