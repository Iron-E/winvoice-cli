use std::{collections::BTreeMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{Adapters, Employees, Error, Invoices, Jobs, Result, Store, StoreValue};

/// The data type backing a user's configuration.
///
/// # Examples
///
/// ## TOML
///
/// ```rust
/// # assert!(toml::from_str::<clinvoice_config::Config>(r#"
/// [employees]
/// id = 1
/// organization_id = 2
///
/// [invoices]
/// default_currency = "USD"
///
/// [jobs]
/// default_increment = "15min"
///
/// [stores]
/// default = "foo"
///
/// [stores.foo]
/// adapter = "postgres"
/// url = "postgres://username:password@localhost:5432/database_name"
/// # "#).is_ok());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Config
{
	/// The [`[employees]`](Employees) field.
	#[serde(default)]
	pub employees: Employees,

	/// The [`[invoices]`](Invoices) field.
	#[serde(default)]
	pub invoices: Invoices,

	/// The [`[jobs]`](Jobs) field.
	#[serde(default)]
	pub jobs: Jobs,

	/// The `[stores]` field, which dictates the [`Store`]s that CLInvoice may operate on. Keyed
	/// on the label of the [`Store`].
	///
	/// The [`Store`] used by default should be labelled "default". [`Config::init`] will generate a
	/// label with this name.
	///
	/// # Notes
	///
	/// * This is a [`BTreeMap`] because, since it is sorted, it guarantees serialization in a
	///   consistent order.
	#[serde(default)]
	stores: BTreeMap<String, StoreValue>,
}

impl Config
{
	/// Get the [`Store`] labelled `name`, resolving any [`StoreValue::Alias`]es which `name` may
	/// point to.
	///
	/// Returns [`None`] if no [`Store`] labelled `name` could be found.
	///
	/// # Examples
	///
	/// ```rust
	/// use core::time::Duration;
	///
	/// use clinvoice_config::{Adapters, Config, Employees, Invoices, Jobs, Store, StoreValue};
	/// use clinvoice_schema::Currency;
	/// # use pretty_assertions::{assert_eq, assert_ne};
	///
	/// let conf: Config = toml::from_str(r#"
	///   [stores]
	///   a = "b"
	///   b = "c"
	///   c = {adapter = "postgres", url = "c/path"}
	///   d = {adapter = "postgres", url = "d/path"}
	///   e = "d"
	/// "#).unwrap();
	///
	/// // Reflexivity
	/// assert_eq!(
	///   conf.get_store("a").as_deref(),
	///   conf.get_store("b").as_deref()
	/// );
	/// assert_eq!(
	///   conf.get_store("b").as_deref(),
	///   conf.get_store("c").as_deref()
	/// );
	/// assert_eq!(
	///   conf.get_store("a").as_deref(),
	///   conf.get_store("c").as_deref()
	/// );
	/// assert_eq!(
	///   conf.get_store("d").as_deref(),
	///   conf.get_store("e").as_deref()
	/// );
	///
	/// // Should never be the same
	/// assert_ne!(
	///   conf.get_store("c").as_deref(),
	///   conf.get_store("d").as_deref()
	/// );
	/// assert_ne!(
	///   conf.get_store("a").as_deref(),
	///   conf.get_store("e").as_deref()
	/// );
	/// ```
	pub fn get_store(&self, name: &str) -> Option<&Store>
	{
		self.stores.get(name).and_then(|value| match value
		{
			StoreValue::Alias(alias) => self.get_store(alias),
			StoreValue::Storage(store) => Some(store),
		})
	}

	/// [Write](Config::write) and return a configuration file with some defaults.
	///
	/// # Warnings
	///
	/// * This function _will_ clobber an existing configuration! Check that [`Config::path`] is not
	///   [a file](std::path::Path::is_file) before use.
	fn init() -> Result<Self>
	{
		let config = Self {
			stores: [
				("default".into(), StoreValue::Alias("foo".into())),
				(
					"foo".into(),
					StoreValue::Storage(Store {
						adapter: Adapters::Postgres,
						url: "See https://github.com/Iron-E/clinvoice/wiki/Usage#adapters".into(),
					}),
				),
			]
			.into_iter()
			.collect(),
			..Default::default()
		};

		config.write()?;

		Ok(config)
	}

	/// Read the configuration from the [`Config::path`], or [`Config::init`] it if [`Config::path`]
	/// does not exist.
	pub fn read() -> Result<Self>
	{
		let path = Self::path();

		if !path.is_file()
		{
			return Config::init();
		}

		let config_bytes = fs::read(path)?;
		toml::from_slice(&config_bytes).map_err(Error::from)
	}

	/// The place where a user [`Config`] should be stored on the hard drive.
	fn path() -> PathBuf
	{
		dirs::config_dir()
			.expect("Operating System is not supported")
			.join("clinvoice")
			.join("config.toml")
	}

	/// Save an in-memory [`Config`] to disk at the [`Config::path`].
	pub fn write(&self) -> Result<()>
	{
		let path = Self::path();

		// TODO: use if-let chains
		if let Some(parent) = path.parent()
		{
			if !parent.is_dir()
			{
				fs::create_dir_all(parent)?;
			}
		}

		let serialized = toml::to_string_pretty(self)?;
		fs::write(path, serialized).map_err(Error::from)
	}
}
