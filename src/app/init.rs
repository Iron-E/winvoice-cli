use clinvoice_adapter::Initializable;
use clinvoice_config::{Adapters, Error, Store};
#[cfg(feature = "postgres")]
use {
	clinvoice_adapter_postgres::PgSchema,
	sqlx::{Connection, PgConnection},
};

use crate::DynResult;

/// # Summary
///
/// Execute the constructed command.
pub async fn run(store: &Store) -> DynResult<()>
{
	match store.adapter
	{
		#[cfg(feature = "postgres")]
		Adapters::Postgres =>
		{
			let mut connection = PgConnection::connect(&store.url).await?;
			PgSchema::init(&mut connection).await?;
		},

		// NOTE: this is allowed because there may be additional adapters added later, and I want
		//       to define this behavior now.
		#[allow(unreachable_patterns)]
		_ => return Err(Error::FeatureNotFound(store.adapter).into()),
	};

	Ok(())
}
