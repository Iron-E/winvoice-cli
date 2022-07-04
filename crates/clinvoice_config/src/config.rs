use std::{collections::BTreeMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{Adapters, Employees, Error, Invoices, Result, Store, StoreValue, Timesheets};

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
///
/// TODO: see if the number of lifetime params can be reduced
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Config
{
	/// # Summary
	///
	/// Configurations for [`Employee`](clinvoice_schema::employee::Employee)s.
	#[serde(default)]
	pub employees: Employees,

	/// # Summary
	///
	/// Configurations for [`Invoice`](clinvoice_schema::invoice::Invoice)s.
	#[serde(default)]
	pub invoices: Invoices,

	/// # Summary
	///
	/// Configurations for data storages.
	///
	/// NOTE: this is a [`BTreeMap`] because it is desirable for configuration files to be
	///       serialized in a consistent order.
	#[serde(default)]
	stores: BTreeMap<String, StoreValue>,

	/// # Summary
	///
	/// Configurations for [`Timesheet`](clinvoice_schema::timesheet:Timesheet)s.
	#[serde(default)]
	pub timesheets: Timesheets,
}

impl Config
{
	/// # Summary
	///
	/// Get the [`Store`] from `name`, resolving any [`StoreValue::Alias`] which `name` may point to.
	///
	/// # Parameters
	///
	/// * `name`, the name of the [`Store`] which should be returned.
	///
	/// # Returns
	///
	/// The [`Store`] which corresponds to `name`.
	///
	/// # Examples
	///
	/// ```rust
	/// use core::time::Duration;
	/// use clinvoice_config::{Adapters, Config, Employees, Invoices, Store, StoreValue, Timesheets};
	/// use clinvoice_schema::Currency;
	///
	/// let conf: Config = toml::from_str(r#"
	///   [employees]
	///   id = 1
	///   organization_id = 2
	///
	///   [invoices]
	///   default_currency = "USD"
	///
	///   [stores]
	///   a = "b"
	///   b = "c"
	///   c = {adapter = "Postgres", url = "c/path"}
	///   d = {adapter = "Postgres", url = "d/path"}
	///   e = "d"
	///
	///   [timesheets]
	///   default_increment = "100s"
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

	/// # Summary
	///
	/// Create a configuration file with some defaults.
	pub fn init() -> Result<()>
	{
		let path = Self::path();

		if path.is_file()
		{
			return Ok(());
		}

		// TODO: use if-let chains
		if let Some(parent) = path.parent()
		{
			if !parent.is_dir()
			{
				fs::create_dir_all(parent)?;
			}
		}

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

		config.update()
	}

	pub fn path() -> PathBuf
	{
		dirs::config_dir()
			.expect("Operating System is not supported")
			.join("clinvoice")
			.join("config.toml")
	}

	pub fn update(&self) -> Result<()>
	{
		let serialized = toml::to_string_pretty(self)?;
		fs::write(Self::path(), serialized).map_err(Error::from)
	}
}
