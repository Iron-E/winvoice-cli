use clinvoice_schema::{views::LocationView, Id};
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::{Executor, Postgres, Result};

mod deletable;
mod location_adapter;
mod updatable;

pub struct PostgresLocation;

impl PostgresLocation
{
	pub(super) async fn retrieve_view_by_id(
		connection: impl Executor<'_, Database = Postgres>,
		id: Id,
	) -> Result<LocationView>
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
		.try_fold(None, |previous: Option<LocationView>, view| {
			future::ok(Some(LocationView {
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
