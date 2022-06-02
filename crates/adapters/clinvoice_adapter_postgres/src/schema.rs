//! # Summary
//!
//! This module implements adapters (and associated adapter types such as
//! [`Deletable`](clinvoice_adapter::Deletable)) for a Postgres filesystem.

mod contact_info;
mod employee;
mod expenses;
mod initializable;
mod interval;
mod job;
mod location;
mod option;
mod organization;
mod scoped_column;
mod timesheet;
mod timestamptz;
mod typecast;
mod util;
mod write_where_clause;

use core::fmt::Display;

use clinvoice_adapter::WriteWhereClause;
use clinvoice_match::Match;
use clinvoice_schema::Id;
pub use contact_info::PgContactInfo;
pub use employee::PgEmployee;
pub use expenses::PgExpenses;
pub(crate) use interval::PgInterval;
pub use job::PgJob;
pub use location::PgLocation;
pub(crate) use option::PgOption;
pub use organization::PgOrganization;
pub(crate) use scoped_column::PgScopedColumn;
use sqlx::{Executor, Postgres, QueryBuilder, Result};
pub use timesheet::PgTimesheet;
pub(crate) use timestamptz::PgTimestampTz;

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
		let mut query = QueryBuilder::new("DELETE FROM ");
		query.push(table);

		PgSchema::write_where_clause(
			Default::default(),
			"id",
			&Match::Or(entities.map(Match::from).collect()),
			&mut query,
		);

		query.push(';').build().execute(connection).await?;

		Ok(())
	}
}
