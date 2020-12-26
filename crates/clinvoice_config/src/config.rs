mod employees;
mod invoices;
mod store_value;
mod timesheets;

use clinvoice_adapter::Store;
pub use self::{employees::Employees, invoices::Invoices, store_value::StoreValue, timesheets::Timesheets};

use std::collections::HashMap;

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
pub struct Config<'alias, 'name, 'pass, 'path, 'user>
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
	stores: HashMap<&'name str, StoreValue<'alias, 'pass, 'path, 'user>>,

	/// # Summary
	///
	/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
	pub timesheets: Timesheets,
}

impl Config<'_, '_, '_, '_, '_>
{
	pub fn get_store(&self, name: &str) -> Option<&Store<'_, '_, '_>>
	{
		return match self.stores.get(name)
		{
			None => None,
			Some(value) => match value
			{
				StoreValue::Alias(alias) => self.get_store(alias),
				StoreValue::Storage(store) => Some(store),
			},
		};

	}
}
