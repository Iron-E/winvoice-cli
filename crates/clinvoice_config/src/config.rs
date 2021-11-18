mod error;

use std::{collections::BTreeMap, fs, path::PathBuf};

use clinvoice_adapter::{Adapters, Store};
pub use error::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::{Employees, Invoices, StoreValue, Timesheets};

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
///
/// TODO: see if the number of lifetime params can be reduced
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Config<'alias, 'name>
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
	#[serde(borrow)]
	stores: BTreeMap<&'name str, StoreValue<'alias>>,

	/// # Summary
	///
	/// Configurations for [`Timesheet`](clinvoice_schema::timesheet:Timesheet)s.
	#[serde(default)]
	pub timesheets: Timesheets,
}

impl Config<'_, '_>
{
	/// # Summary
	///
	/// Create a configuration file with some defaults.
	pub fn init() -> Result<()>
	{
		let path = Self::path();
		if !path.is_file()
		{
			if let Some(parent) = path.parent()
			{
				if !parent.is_dir()
				{
					fs::create_dir_all(parent)?;
				}
			}

			let config = Self {
				employees: Default::default(),
				invoices: Default::default(),
				stores: vec![
					("default", StoreValue::Alias("foo")),
					(
						"foo",
						StoreValue::Storage(Store {
							adapter: Adapters::Postgres,
							url: "See https://github.com/Iron-E/clinvoice/wiki/Usage#adapters".into(),
						}),
					),
				]
				.into_iter()
				.collect(),
				timesheets: Default::default(),
			};

			config.update()?;
		}

		Ok(())
	}

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
	pub fn get_store(&self, name: &str) -> Option<&Store>
	{
		self.stores.get(name).and_then(|value| match value
		{
			StoreValue::Alias(alias) => self.get_store(alias),
			StoreValue::Storage(store) => Some(store),
		})
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
		fs::write(Self::path(), serialized)?;

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;

	use clinvoice_adapter::Adapters;
	use clinvoice_schema::{Currency, Id};

	use super::{BTreeMap, Config, Employees, Invoices, Store, StoreValue, Timesheets};

	#[test]
	fn get_store()
	{
		let mut stores = BTreeMap::new();

		stores.insert("a", StoreValue::Alias("b"));
		stores.insert("b", StoreValue::Alias("c"));
		stores.insert(
			"c",
			StoreValue::Storage(Store {
				adapter: Adapters::Postgres,
				url: "c/path".into(),
			}),
		);
		stores.insert(
			"d",
			StoreValue::Storage(Store {
				adapter: Adapters::Postgres,
				url: "d/path".into(),
			}),
		);
		stores.insert("e", StoreValue::Alias("d"));

		let conf = Config {
			employees: Employees {
				default_id: Some(Id::default()),
			},
			invoices: Invoices {
				default_currency: Currency::USD,
			},
			stores,
			timesheets: Timesheets {
				default_increment: Duration::new(100, 0),
			},
		};

		// Reflexivity
		assert_eq!(
			conf.get_store("a").as_deref(),
			conf.get_store("b").as_deref()
		);
		assert_eq!(
			conf.get_store("b").as_deref(),
			conf.get_store("c").as_deref()
		);
		assert_eq!(
			conf.get_store("a").as_deref(),
			conf.get_store("c").as_deref()
		);
		assert_eq!(
			conf.get_store("d").as_deref(),
			conf.get_store("e").as_deref()
		);

		// Should never be the same
		assert_ne!(
			conf.get_store("c").as_deref(),
			conf.get_store("d").as_deref()
		);
		assert_ne!(
			conf.get_store("a").as_deref(),
			conf.get_store("e").as_deref()
		);
	}
}
