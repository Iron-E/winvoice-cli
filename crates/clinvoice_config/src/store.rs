use serde::{Deserialize, Serialize};

use crate::Adapters;

/// A place for CLInvoice to store information.
///
/// The storage should be set up by the [`Organization`](clinvoice_schema::Organization)'s IT
/// administrator, taking care to provide a valid configuration for users.
///
/// # Example
///
/// ```rust
/// use clinvoice_config::{Adapters, Store};
///
/// let _ = Store {
///   adapter: Adapters::Postgres,
///   url: "postgres://username:password@localhost:5432/database_name".into(),
/// };
/// ```
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Store
{
	/// The type of storage system being used to store information.
	pub adapter: Adapters,

	/// The URL where CLInvoice can communicate with the storage. This setting is highly dependent
	/// on the `adapter` which was chosen; see [the
	/// docs](https://github.com/Iron-E/clinvoice/wiki/Usage#adapters) for more information.
	pub url: String,
}
