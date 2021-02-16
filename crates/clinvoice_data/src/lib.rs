mod contact;
mod employee;
mod employee_status;
mod expense;
mod expense_category;
mod id;
mod invoice;
mod invoice_date;
mod job;
mod location;
mod money;
mod organization;
mod person;
mod timesheet;

pub use
{
	contact::Contact,
	employee::Employee,
	employee_status::EmployeeStatus,
	expense::Expense,
	expense_category::ExpenseCategory,
	id::Id,
	invoice::Invoice,
	invoice_date::InvoiceDate,
	job::Job,
	location::Location,
	money::Money,
	organization::Organization,
	person::Person,
	timesheet::Timesheet,
};

pub use chrono;
pub use rust_decimal::Decimal;

/// # Summary
///
/// The namespace for a v5 [`Uuid`](uuid::Uuid) containing CLInvoice data.
pub const UUID_NAMESPACE: Id = Id::from_bytes([
	0x1a, 0x88, 0xb1, 0xde,
	0xe8, 0x0d, 0x4e, 0xca,
	0x92, 0x08, 0xe5, 0x6b,
	0x09, 0x9a, 0x6f, 0x4b
]);
