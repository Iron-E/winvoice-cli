mod employees;
mod invoices;
mod store_value;
mod timesheets;

pub use self::{employees::Employees, invoices::Invoices, store_value::StoreValue, timesheets::Timesheets};

use std::collections::HashMap;

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
pub struct Config<'alias, 'db, 'name, 'pass, 'path, 'user>
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
	stores: HashMap<&'name str, StoreValue<'alias, 'db, 'pass, 'path, 'user>>,

	/// # Summary
	///
	/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
	pub timesheets: Timesheets,
}

impl Config
{
	pub fn get_storage(&self, name: &str) -> Option<StoreValue>
	{
		let value = match self.stores.get(name)
		{
			Some(v) => v,
			None => return None,
		};

		return match value
		{
			Database(d) => d,
			FileSystem(fs) => self.get_storage(fs),
		};
	}
}
