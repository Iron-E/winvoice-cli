pub mod employees;
pub mod employers;
pub mod invoices;
pub mod timesheets;

use self::{employees::Employees, employers::Employers, invoices::Invoices, timesheets::Timesheets};

/// # Summary
///
/// The `Config` contains settings that affect all areas of the application.
pub struct Config
{
	/// # Summary
	///
	/// Configurations for [`Employee`](clinvoice_data::employee::Employee)s.
	pub employees: Employees,

	/// # Summary
	///
	/// Configurations for [`Employer`](clinvoice_data::employer::Employer)s.
	pub employers: Employers,

	/// # Summary
	///
	/// Configurations for [`Invoice`](clinvoice_data::invoice::Invoice)s.
	pub invoices: Invoices,

	/// # Summary
	///
	/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
	pub timesheets: Timesheets,
}
