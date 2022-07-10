use serde::{Deserialize, Serialize};

use crate::Store;

/// Possible values for the `[store]` field of the [user config](crate::Config).
///
/// # Example
///
/// ## TOML
///
/// The following creates a storage named "a", and sets it as the default:
///
/// ```rust
/// # assert!(toml::from_str::<std::collections::HashMap<String, clinvoice_config::StoreValue>>(r#"
/// default = "a"
///
/// [a]
/// adapter = "postgres"
/// url = "a/path"
/// # "#).is_ok());
/// ```
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum StoreValue
{
	/// A link to another `[store]` field.
	Alias(String),

	/// A [`Store`] specification.
	Storage(Store),
}
