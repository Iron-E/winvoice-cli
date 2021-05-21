mod error;
mod updatable;

pub use error::{Error, Result};

use
{
	core::time::Duration,
	std::{collections::BTreeMap, path::PathBuf},

	crate::{Employees, Invoices, StoreValue, Timesheets},

	clinvoice_adapter::{Adapters, data::Updatable, Store},
	clinvoice_data::Id,

	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Config<'alias, 'currency, 'name>
{
	/// # Summary
	///
	/// Configurations for [`Employee`](clinvoice_data::employee::Employee)s.
	pub employees: Employees,

	/// # Summary
	///
	/// Configurations for [`Invoice`](clinvoice_data::invoice::Invoice)s.
	#[serde(borrow)]
	pub invoices: Invoices<'currency>,

	/// # Summary
	///
	/// Configurations for data storages.
	#[serde(borrow)]
	stores: BTreeMap<&'name str, StoreValue<'alias>>,

	/// # Summary
	///
	/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
	pub timesheets: Timesheets,
}

impl Config<'_, '_, '_>
{
	/// # Summary
	///
	/// Create a configuration file with some defaults.
	pub fn init() -> Result<()>
	{
		if !Self::path().is_file()
		{
			let config = Self
			{
				employees: Employees {default_id: Id::default()},
				invoices: Invoices {default_currency: "USD"},
				stores: vec![
					("default", StoreValue::Alias("foo")),
					("foo", StoreValue::Storage(Store
					{
						adapter: Adapters::Bincode,
						password: Some("Optional password. May or may not be accompanied by a username".into()),
						username: Some("Optional username. May or may not be accompanied by a password".into()),
						path: "Place where data can be found. Depends on the adapter— may be a path to a folder on a filesystem, or a schema on a database".into(),
					})),
				].into_iter().collect(),
				timesheets: Timesheets {interval: Duration::from_secs(300)},
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
		dirs::config_dir().expect("Operating System is not supported").join("clinvoice").join("config.toml")
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::time::{Duration, Instant},

		super::{Config, Employees, BTreeMap, Invoices, Store, StoreValue, Timesheets},

		clinvoice_adapter::Adapters,
		clinvoice_data::Id,
	};

	#[test]
	fn get_store()
	{
		let mut stores = BTreeMap::new();

		stores.insert("a", StoreValue::Alias("b"));
		stores.insert("b", StoreValue::Alias("c"));
		stores.insert("c", StoreValue::Storage(Store {
			adapter: Adapters::Bincode,
			password: None,
			path: "c/path".into(),
			username: None,
		}));
		stores.insert("d", StoreValue::Storage(Store {
			adapter: Adapters::Bincode,
			password: Some("asldkj".into()),
			path: "d/path".into(),
			username: None,
		}));
		stores.insert("e", StoreValue::Alias("d"));

		let conf = Config
		{
			employees: Employees {default_id: Id::new_v4()},
			invoices: Invoices {default_currency: "USD"},
			stores,
			timesheets: Timesheets {interval: Duration::new(100, 0)},
		};

		let start = Instant::now();
		// Reflexivity
		assert_eq!(conf.get_store("a").as_deref(), conf.get_store("b").as_deref());
		assert_eq!(conf.get_store("b").as_deref(), conf.get_store("c").as_deref());
		assert_eq!(conf.get_store("a").as_deref(), conf.get_store("c").as_deref());
		assert_eq!(conf.get_store("d").as_deref(), conf.get_store("e").as_deref());

		// Should never be the same
		assert_ne!(conf.get_store("c").as_deref(), conf.get_store("d").as_deref());
		assert_ne!(conf.get_store("a").as_deref(), conf.get_store("e").as_deref());

		println!("\n>>>>> Config::get_store {}us <<<<<\n", Instant::now().duration_since(start).as_micros() / 12);
	}
}
