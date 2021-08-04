mod error;
mod updatable;

use core::time::Duration;
use std::{
	collections::BTreeMap,
	path::PathBuf,
};

use clinvoice_adapter::{
	data::Updatable,
	Adapters,
	Store,
};
use clinvoice_data::{
	finance::Currency,
	Id,
};
pub use error::{
	Error,
	Result,
};
use serde::{
	Deserialize,
	Serialize,
};

use crate::{
	Employees,
	Invoices,
	StoreValue,
	Timesheets,
};

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Config<'alias, 'name>
{
	/// # Summary
	///
	/// Configurations for [`Employee`](clinvoice_data::employee::Employee)s.
	pub employees: Employees,

	/// # Summary
	///
	/// Configurations for [`Invoice`](clinvoice_data::invoice::Invoice)s.
	pub invoices: Invoices,

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

impl Config<'_, '_>
{
	/// # Summary
	///
	/// Create a configuration file with some defaults.
	pub async fn init() -> Result<()>
	{
		if !Self::path().is_file()
		{
			let config = Self {
				employees:  Employees {
					default_id: Id::default(),
				},
				invoices:   Invoices {
					default_currency: Currency::USD,
				},
				stores:     vec![
					("default", StoreValue::Alias("foo")),
					(
						"foo",
						StoreValue::Storage(Store {
							adapter:  Adapters::Bincode,
							password: Some(
								"Optional password. May or may not be accompanied by a username".into(),
							),
							username: Some(
								"Optional username. May or may not be accompanied by a password".into(),
							),
							path:     "See https://github.com/Iron-E/clinvoice/wiki/Usage#adapters".into(),
						}),
					),
				]
				.into_iter()
				.collect(),
				timesheets: Timesheets {
					interval: Duration::from_secs(300),
				},
			};

			config.update().await?;
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
}

#[cfg(test)]
mod tests
{
	use std::time::{
		Duration,
		Instant,
	};

	use clinvoice_adapter::Adapters;
	use clinvoice_data::Id;

	use super::{
		BTreeMap,
		Config,
		Currency,
		Employees,
		Invoices,
		Store,
		StoreValue,
		Timesheets,
	};

	#[test]
	fn get_store()
	{
		let mut stores = BTreeMap::new();

		stores.insert("a", StoreValue::Alias("b"));
		stores.insert("b", StoreValue::Alias("c"));
		stores.insert(
			"c",
			StoreValue::Storage(Store {
				adapter:  Adapters::Bincode,
				password: None,
				path:     "c/path".into(),
				username: None,
			}),
		);
		stores.insert(
			"d",
			StoreValue::Storage(Store {
				adapter:  Adapters::Bincode,
				password: Some("asldkj".into()),
				path:     "d/path".into(),
				username: None,
			}),
		);
		stores.insert("e", StoreValue::Alias("d"));

		let conf = Config {
			employees: Employees {
				default_id: Id::new_v4(),
			},
			invoices: Invoices {
				default_currency: Currency::USD,
			},
			stores,
			timesheets: Timesheets {
				interval: Duration::new(100, 0),
			},
		};

		let start = Instant::now();
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

		println!(
			"\n>>>>> Config::get_store {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros() / 12
		);
	}
}
