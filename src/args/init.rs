use clap::Args as Clap;
use clinvoice_adapter::Initializable;
use clinvoice_config::{Adapters, Config, Error};
use sqlx::Connection;

use super::store_args::StoreArgs;
use crate::DynResult;

#[derive(Clap, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Init
{
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

			// NOTE: this is allowed because there may be additional adapters added later, and I want
			//       to define this behavior now.
			#[allow(unreachable_patterns)]
			_ => return Err(Error::FeatureNotFound(store.adapter).into()),
		};

		Ok(())
	}
}
