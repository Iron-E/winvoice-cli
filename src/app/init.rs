use clinvoice_adapter::{data::Initializable, Adapters, Error as AdapterError, Store};
#[cfg(feature = "postgres")]
use {clinvoice_adapter_postgres::data::PostgresSchema, sqlx::PgPool};

use crate::DynResult;

/// # Summary
///
/// Execute the constructed command.
pub async fn run<'err>(store: &Store) -> DynResult<'err, ()>
{
	match store.adapter
	{
		#[cfg(feature = "postgres")]
		Adapters::Postgres =>
		{
			let pool = PgPool::connect_lazy(&store.url)?;
			PostgresSchema::init(&pool).await?;
		},

		// NOTE: this is allowed because there may be additional adapters added later, and I want
		//       to define this behavior now.
		#[allow(unreachable_patterns)]
		_ => return Err(AdapterError(store.adapter).into()),
	};

	Ok(())
}
