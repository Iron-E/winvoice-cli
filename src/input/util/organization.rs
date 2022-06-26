use core::fmt::Display;

use clinvoice_adapter::{schema::OrganizationAdapter, Deletable};
use clinvoice_match::MatchOrganization;
use clinvoice_schema::Organization;
use sqlx::{Database, Executor, Pool};

use super::{menu, MATCH_PROMPT};
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
/// [P_retrieve]: clinvoice_adapter::schema::OrganizationAdapter::retrieve
/// [organization]: clinvoice_schema::Organization
pub async fn retrieve<D, Db, OAdapter>(
	connection: &Pool<Db>,
	prompt: D,
	retry_on_empty: bool,
) -> DynResult<Vec<Organization>>
where
	D: Display,
	Db: Database,
	OAdapter: Deletable<Db = Db> + OrganizationAdapter,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	loop
	{
		let match_condition: MatchOrganization =
			input::edit_default(format!("{prompt}\n{MATCH_PROMPT}organizations"))?;

		let results = OAdapter::retrieve(connection, &match_condition).await?;

		if retry_on_empty && results.is_empty() && menu::ask_to_retry()?
		{
			continue;
		}

		return Ok(results);
	}
}
