use clinvoice_adapter::{Updatable, WriteWhereClause};
use clinvoice_match::Match;
use clinvoice_schema::Location;
use sqlx::{Postgres, QueryBuilder, Result, Transaction};

use super::PgLocation;
use crate::{
	schema::{location::columns::PgLocationColumns, PgOption},
	PgSchema,
};

#[async_trait::async_trait]
impl Updatable for PgLocation
{
	type Db = Postgres;
	type Entity = Location;

	async fn update(connection: &mut Transaction<Self::Db>, entity: Self::Entity) -> Result<()>
	{
		const COLUMNS: PgLocationColumns<'static> = PgLocationColumns::new();

		let mut query = QueryBuilder::new("UPDATE locations SET ");

		{
			let mut separated = query.separated(' ');

			separated
				.push(COLUMNS.name)
				.push('=')
				.push_bind(entity.name)
				.push(',')
				.push(COLUMNS.outer_id)
				.push('=')
				.push(PgOption(entity.outer.as_ref().map(|o| o.id)));
		}

		PgSchema::write_where_clause(
			Default::default(),
			COLUMNS.id,
			&Match::from(entity.id),
			&mut query,
		);

		query.push(';').build().execute(&mut *connection).await?;

		if let Some(o) = entity.outer
		{
			Self::update(connection, *o).await?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn update()
	{
		// TODO: write test
	}
}
