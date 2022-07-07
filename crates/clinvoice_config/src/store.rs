use serde::{Deserialize, Serialize};

use crate::Adapters;

/// A place for CLInvoice to store information.
///
/// The storage should be set up by the [`Organization`](clinvoice_schema::Organization)'s IT
/// administrator, taking care to provide a valid configuration for users.
///
/// # Example
///
/// ## TOML
///
/// For the [postgres adapter](Adapters::Postgres):
///
/// ```rust
/// # assert!(toml::from_str::<clinvoice_config::Store>(r#"
/// adapter = "postgres"
/// url = "postgres://username:password@localhost:5432/database_name"
/// # "#).is_ok());
/// ```
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Store
{
	/// The type of storage system being used to store information.
	pub adapter: Adapters,

	/// The URL where CLInvoice can communicate with the storage. This setting is highly dependent
	/// on the `adapter` which was chosen:
	///
	/// * [`Postgres`](Adapters::Postgres): the connection URI per
	///   [https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING]
	pub url: String,
}
