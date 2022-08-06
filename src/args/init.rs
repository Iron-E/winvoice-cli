use clap::Args as Clap;
use clinvoice_adapter::Initializable;
use clinvoice_config::{Adapters, Config, Error};
use sqlx::Connection;

use super::store_args::StoreArgs;
use crate::DynResult;

/// Prepare the specified store (-s) for use with CLInvoice.
///
/// Will not clobber existing data. Should only be run by administrators.
#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Init
{
	/// Specifies the [`Store`](clinvoice_config::Store) to [`Init`].
	#[clap(flatten)]
	store_args: StoreArgs,
}

impl Init
{
	/// # Summary
	///
	/// Execute the constructed command.
	pub async fn run(self, config: &Config) -> DynResult<()>
	{
		let store = self.store_args.try_get_from(config)?;

		match store.adapter
		{
			#[cfg(feature = "postgres")]
			Adapters::Postgres =>
			{
				use clinvoice_adapter_postgres::PgSchema;
				use sqlx::PgConnection;

				let mut connection = PgConnection::connect(&store.url).await?;
				PgSchema::init(&mut connection).await?;
			},

			// NOTE: this is allowed because there may be additional adapters added later, and I
			// want       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => return Err(Error::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}
