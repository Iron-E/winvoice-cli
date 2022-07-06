use serde::{Deserialize, Serialize};

use crate::Store;

/// Possible values for the `[store]` field of the [user config](crate::Config).
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use clinvoice_config::{Adapters, Store, StoreValue};
///
/// let values: HashMap<String, StoreValue> = toml::from_str(r#"
///   default = "a"
///   a = {adapter = "postgres", url = "a/path"}
/// "#).unwrap();
///
/// assert_eq!(values["default"], StoreValue::Alias("a".into()));
/// assert_eq!(values["a"], StoreValue::Storage(Store {
///   adapter: Adapters::Postgres,
///   url: "a/path".into(),
/// }));
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
