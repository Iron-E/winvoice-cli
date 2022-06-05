//! # Summary
//!
//! This module implements adapters (and associated adapter types such as
//! [`Deletable`](clinvoice_adapter::Deletable)) for a Postgres filesystem.

mod contact_info;
mod employee;
mod expenses;
mod initializable;
mod job;
mod location;
mod organization;
mod timesheet;
mod util;
mod write_where_clause;

use core::fmt::Display;

use clinvoice_adapter::WriteWhereClause;
use clinvoice_match::Match;
use clinvoice_schema::Id;
pub use contact_info::PgContactInfo;
pub use employee::PgEmployee;
pub use expenses::PgExpenses;
pub use job::PgJob;
pub use location::PgLocation;
pub use organization::PgOrganization;
use sqlx::{Executor, Postgres, QueryBuilder, Result};
pub use timesheet::PgTimesheet;

/// # Summary
///
/// An empty struct which implements [`Initializable`](clinvoice_adapter::schema::Initializable) so
/// that the Postgres database can have all of the necessary tables set up if this is the first run
/// of the program.
pub struct PgSchema;

impl PgSchema
{
	/// # Summary
	///
	/// Generate `DELETE FROM {table} WHERE (id = №) OR … OR (id = №)`
	/// for each [`Id`] in `entities.
	///
	/// Note that a semicolon is not put at the end of the statement.
	async fn delete(
		connection: impl Executor<'_, Database = Postgres>,
		table: impl Display,
		entities: impl Iterator<Item = Id>,
	) -> Result<()>
	{
		let mut peekable_entities = entities.peekable();

		// There is nothing to do
		if peekable_entities.peek().is_none()
		{
			return Ok(());
		}

		let mut query = QueryBuilder::new("DELETE FROM ");
		query.push(table);

		PgSchema::write_where_clause(
			Default::default(),
			"id",
			&Match::Or(peekable_entities.map(Match::from).collect()),
			&mut query,
		);

		query.push(';').build().execute(connection).await?;

		Ok(())
	}
}
