use core::fmt::Display;

use clinvoice_adapter::{schema::EmployeeAdapter, Deletable};
use clinvoice_match::MatchEmployee;
use clinvoice_schema::Employee;
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
use crate::{input, DynResult};

/// Retrieve all [`Employee`][location]s from the specified `store` that match a user-provided
/// query. If `RETRY_ON_EMPTY`, the query is attempted again when the query returns no results.
pub async fn retrieve<EAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<Employee>>
where
	EAdapter: Deletable<Db = TDb> + EmployeeAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	loop
	{
		let match_condition: MatchEmployee =
			input::edit_default(format!("{prompt}\n{MATCH_PROMPT}locations"))?;

		let results = EAdapter::retrieve(connection, &match_condition).await?;

		if RETRY_ON_EMPTY && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}

/// [Retrieve](retrieve) `Employee`s and then [select](input::select) from them.
pub async fn select<EAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<Employee>>
where
	EAdapter: Deletable<Db = TDb> + EmployeeAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let locations = retrieve::<EAdapter, TDb, TPrompt, RETRY_ON_EMPTY>(connection, prompt).await?;
	let selected = input::select(&locations, "Select the `Employee`s")?;

	Ok(selected)
}

/// [Retrieve](retrieve) `Employee`s and then [select one](input::select_one) of them.
pub async fn select_one<EAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Employee>
where
	EAdapter: Deletable<Db = TDb> + EmployeeAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let locations = retrieve::<EAdapter, TDb, TPrompt, RETRY_ON_EMPTY>(connection, prompt).await?;
	let selected = input::select_one(&locations, "Select the `Employee`")?;

	Ok(selected)
}
