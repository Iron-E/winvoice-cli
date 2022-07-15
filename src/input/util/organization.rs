use core::fmt::Display;

use clinvoice_adapter::{schema::OrganizationAdapter, Deletable};
use clinvoice_match::MatchOrganization;
use clinvoice_schema::Organization;
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
use crate::{input, DynResult};

/// Retrieve all [`Organization`][location]s from the specified `store` that match a user-provided
/// query. If `RETRY_ON_EMPTY`, the query is attempted again when the query returns no results.
pub async fn retrieve<OAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<Organization>>
where
	OAdapter: Deletable<Db = TDb> + OrganizationAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	loop
	{
		let match_condition: MatchOrganization =
			input::edit_default(format!("{prompt}\n{MATCH_PROMPT}locations"))?;

		let results = OAdapter::retrieve(connection, &match_condition).await?;

		if RETRY_ON_EMPTY && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}

/// [Retrieve](retrieve) `Organization`s and then [select](input::select) from them.
pub async fn select<OAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<Organization>>
where
	OAdapter: Deletable<Db = TDb> + OrganizationAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let locations = retrieve::<OAdapter, TDb, TPrompt, RETRY_ON_EMPTY>(connection, prompt).await?;
	let selected = input::select(&locations, "Select the `Organization`s")?;

	Ok(selected)
}

/// [Retrieve](retrieve) `Organization`s and then [select one](input::select_one) of them.
pub async fn select_one<OAdapter, TDb, TPrompt, const RETRY_ON_EMPTY: bool>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Organization>
where
	OAdapter: Deletable<Db = TDb> + OrganizationAdapter,
	TDb: Database,
	TPrompt: Display,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let locations = retrieve::<OAdapter, TDb, TPrompt, RETRY_ON_EMPTY>(connection, prompt).await?;
	let selected = input::select_one(&locations, "Select the `Organization`")?;

	Ok(selected)
}
