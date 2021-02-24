mod employees;
mod invoices;
mod store_value;
mod timesheets;
mod updatable;

pub use {employees::Employees, invoices::Invoices, store_value::StoreValue, timesheets::Timesheets};

use
{
	clinvoice_adapter::{DynamicResult, data::Updatable, Store},
	clinvoice_data::Id,
	std::{collections::BTreeMap, path::PathBuf, time::Duration},
	serde::{Deserialize, Serialize},
};

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Config<'alias, 'currency, 'name, 'pass, 'path, 'user>
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
	stores: BTreeMap<&'name str, StoreValue<'alias, 'pass, 'path, 'user>>,

	/// # Summary
	///
	/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
	pub timesheets: Timesheets,
}

impl Config<'_, '_, '_, '_, '_, '_>
{
	/// # Summary
	///
	/// Create a configuration file with some defaults.
	pub fn init() -> DynamicResult<()>
	{
		if !Self::path().is_file()
		{
			let config = Self
			{
				employees: Employees {default_id: Id::default()},
				invoices: Invoices {default_currency: "USD"},
				stores: BTreeMap::new(),
				timesheets: Timesheets {interval: Duration::from_secs(300)},
			};

			config.update()?;
		}

		return Ok(());
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
	pub fn get_store(&self, name: &str) -> Option<&Store<'_, '_, '_>>
	{
		return match self.stores.get(name)
		{
			Some(value) => match value
			{
				StoreValue::Alias(alias) => self.get_store(alias),
				StoreValue::Storage(store) => Some(store),
			},
			_ => None,
		};
	}

	pub fn path() -> PathBuf
	{
		return dirs::config_dir().expect("Operating System is not supported.").join("clinvoice").join("config.toml");
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::{Config, Employees, BTreeMap, Invoices, Store, StoreValue, Timesheets},
		clinvoice_adapter::Adapters,
		clinvoice_data::Id,
		std::time::{Duration, Instant},
	};

	#[test]
	fn test_get_store()
	{
		let mut stores = BTreeMap::new();

		stores.insert("a", StoreValue::Alias("b"));
		stores.insert("b", StoreValue::Alias("c"));
		stores.insert("c", StoreValue::Storage(Store {
			adapter: Adapters::Bincode,
			password: None,
			path: "c/path",
			username: None,
		}));
		stores.insert("d", StoreValue::Storage(Store {
			adapter: Adapters::Bincode,
			password: Some("asldkj"),
			path: "d/path",
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
