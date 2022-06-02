use clinvoice_adapter::WriteWhereClause;
use clinvoice_match::{Match, MatchLocation, MatchOuterLocation};
use clinvoice_schema::{Id, Location};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{Executor, Postgres, QueryBuilder, Result, Row};

use crate::{schema::location::columns::PgLocationColumns, PgSchema};

pub(super) mod columns;
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
			// NOTE: this scope exists because we want to get rid of the mutable borrow after we're
			//       done with it.
			{
				let mut separated = query.separated(' ');

				separated
					.push(aliases.current)
					.push("as ( SELECT LO.id, LO.name, LO.outer_id FROM locations LO");

				if !aliases.previous.is_empty()
				{
					separated
						.push("JOIN")
						.push(aliases.previous)
						.push("L ON (LO.id = L.outer_id)");
				}
			}

			PgSchema::write_where_clause(
				PgSchema::write_where_clause(
					if match_condition.outer == MatchOuterLocation::None
					{
						PgSchema::write_where_clause(
							Default::default(),
							"LO.outer_id",
							&Match::Not(Match::<i64>::Any.into()),
							query,
						)
					}
					else
					{
						Default::default()
					},
					"LO.id",
					&match_condition.id,
					query,
				),
				"LO.name",
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
							.push(
								", location_report AS ( SELECT L.id, L.name, L.outer_id FROM locations L \
								 JOIN",
							)
							.push(aliases.current)
							.push(
								"LO ON (L.outer_id = LO.id) UNION SELECT L.id, L.name, L.outer_id FROM \
								 locations L JOIN location_report LO ON (L.outer_id = LO.id))",
							);
					}

					query
						.push(" SELECT id FROM ")
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
						.push(';');
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

		const COLUMNS: PgLocationColumns<&'static str> = PgLocationColumns::new();

		query
			.build()
			.fetch(connection)
			.map_ok(|row| row.get::<Id, _>(COLUMNS.id).into())
			.try_collect()
			.map_ok(Match::Or)
			.await
	}

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
			future::ok(Some(Location {
				id: view.id.expect("`locations` table should have non-null ID"),
				name: view
					.name
					.expect("`locations` table should have non-null name"),
				outer: previous.map(Box::new),
			}))
		})
		.map_ok(|v| v.expect("A database object failed to be returned by recursive query"))
		.await
	}
}
