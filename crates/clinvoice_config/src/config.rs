mod employees;
mod employers;
mod invoices;
mod timesheets;

use clinvoice_adapter::Connection;
pub use self::{employees::Employees, employers::Employers, invoices::Invoices, timesheets::Timesheets};

use std::collections::HashMap;

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
pub struct Config<'name, 'url>
{
	/// # Summary
	///
	/// Configurations for database connections.
	pub connections: HashMap<&'name str, Connection<'url>>,

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
	/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
	pub timesheets: Timesheets,
}
