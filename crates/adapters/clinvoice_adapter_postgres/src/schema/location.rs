use clinvoice_adapter::{schema::columns::LocationColumns, WriteWhereClause};
use clinvoice_match::{Match, MatchLocation, MatchOuterLocation};
use clinvoice_schema::{Id, Location};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{Error, Executor, Postgres, QueryBuilder, Result, Row};

use crate::PgSchema;

mod deletable;
mod location_adapter;
mod updatable;

pub struct PgLocation;

impl PgLocation
{
	pub(super) async fn retrieve_matching_ids<'a>(
		connection: impl Executor<'_, Database = Postgres>,
		match_condition: &MatchLocation,
	) -> Result<Match<Id>>
	{
		struct Aliases<'a>
		{
			current: &'a str,
			previous: &'a str,
		}

		const COLUMNS: LocationColumns<&'static str> = LocationColumns::default();
		const INNER_IDENT: &str = "L";
		const OUTER_IDENT: &str = "LO";

		/// # Summary
		///
		/// Generate multiple Common Table Expressions for a recursive query.
		fn generate_cte(
			aliases: Aliases,
			first: bool,
			match_condition: &MatchLocation,
			query: &mut QueryBuilder<Postgres>,
		)
		{
			let inner_columns = COLUMNS.scoped(INNER_IDENT);
			let outer_columns = COLUMNS.scoped(OUTER_IDENT);

			// NOTE: this scope exists because we want to get rid of the mutable borrow after we're
			//       done with it.
			{
				let mut separated = query.separated(' ');

				separated
					.push(aliases.current)
					.push("AS (SELECT")
					.push(outer_columns.id)
					.push_unseparated(',')
					.push_unseparated(outer_columns.name)
					.push_unseparated(',')
					.push_unseparated(outer_columns.outer_id)
					.push("FROM locations")
					.push(OUTER_IDENT);

				if !aliases.previous.is_empty()
				{
					separated
						.push("JOIN")
						.push(aliases.previous)
						.push(INNER_IDENT)
						.push("ON (")
						.push(outer_columns.id)
						.push_unseparated('=')
						.push_unseparated(inner_columns.outer_id)
						.push_unseparated(')');
				}
			}

			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					if match_condition.outer == MatchOuterLocation::None
					{
						PgSchema::write_where_clause(
							Default::default(),
							outer_columns.outer_id,
							&Match::Not(Match::<i64>::Any.into()),
							query,
						)
					}
					else
					{
						Default::default()
					},
					outer_columns.id,
					&match_condition.id,
					query,
				),
				outer_columns.name,
				&match_condition.name,
				query,
			);

			query.push(')');

			match match_condition.outer
			{
				MatchOuterLocation::Always | MatchOuterLocation::None =>
				{
					if !first
					{
						query
							.separated(' ')
							.push(", location_report AS (SELECT")
							.push(inner_columns.id)
							.push_unseparated(',')
							.push_unseparated(inner_columns.name)
							.push_unseparated(',')
							.push_unseparated(inner_columns.outer_id)
							.push("FROM locations")
							.push(INNER_IDENT)
							.push("JOIN")
							.push(aliases.current)
							.push(OUTER_IDENT)
							.push("ON (")
							.push_unseparated(inner_columns.outer_id)
							.push_unseparated('=')
							.push_unseparated(outer_columns.id)
							.push_unseparated(") UNION SELECT")
							.push(inner_columns.id)
							.push_unseparated(',')
							.push_unseparated(inner_columns.name)
							.push_unseparated(',')
							.push_unseparated(inner_columns.outer_id)
							.push("FROM locations")
							.push(INNER_IDENT)
							.push("JOIN location_report")
							.push(OUTER_IDENT)
							.push("ON (")
							.push_unseparated(inner_columns.outer_id)
							.push_unseparated('=')
							.push_unseparated(outer_columns.id)
							.push_unseparated("))");
					}

					query
						.separated(' ')
						.push(" SELECT")
						.push(COLUMNS.id)
						.push("FROM")
						.push(
							if first
							{
								aliases.current
							}
							else
							{
								"location_report"
							},
						)
						.push_unseparated(';');
				},
				MatchOuterLocation::Some(ref outer) =>
				{
					query.push(',');
					generate_cte(
						Aliases {
							current: &format!("{}_outer", aliases.current),
							previous: aliases.current,
						},
						false,
						outer,
						query,
					)
				},
			}
		}

		let mut query = QueryBuilder::new("WITH RECURSIVE ");
		generate_cte(
			Aliases {
				current: "location",
				previous: "",
			},
			true,
			match_condition,
			&mut query,
		);

		query
			.build()
			.fetch(connection)
			.map_ok(|row| row.get::<Id, _>(COLUMNS.id).into())
			.try_collect()
			.map_ok(Match::Or)
			.await
	}

	/// # Summary
	///
	/// Recursively constructs a [`Location`] which matches the `id` in the database over the `connection`.
	///
	/// # Panics
	///
	/// If `id` does not match any row in the database.
	pub(super) async fn retrieve_by_id(
		connection: impl Executor<'_, Database = Postgres>,
		id: Id,
	) -> Result<Location>
	{
		sqlx::query!(
			"WITH RECURSIVE location_view AS
			(
				SELECT id, name, outer_id FROM locations WHERE id = $1
				UNION
				SELECT L.id, L.name, L.outer_id FROM locations L JOIN location_view V ON (L.id = V.outer_id)
			) SELECT * FROM location_view ORDER BY id;",
			id,
		)
		.fetch(connection)
		.try_fold(None, |previous: Option<Location>, view| {
			let id = match view.id
			{
				Some(id) => id,
				_ =>
				{
					return future::err(Error::ColumnDecode {
						index: "id".into(),
						source: "this column in `locations` must be non-null".into(),
					})
				},
			};

			let name = match view.name
			{
				Some(n) => n,
				_ =>
				{
					return future::err(Error::ColumnDecode {
						index: "name".into(),
						source: "this column in `locations` must be non-null".into(),
					})
				},
			};

			future::ok(Some(Location {
				id,
				name,
				outer: previous.map(Box::new),
			}))
		})
		.map_ok(|v| v.expect("`id` did not match any row in the database"))
		.await
	}
}
