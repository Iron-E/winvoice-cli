use std::borrow::Cow::Borrowed;

use clinvoice_data::{
	Employee,
	Timesheet,
};
use clinvoice_query as query;

use super::{
	EmployeeAdapter,
	Error,
};
use crate::Store;

/// # Summary
///
/// Convert some `timesheet` into its referenced [`Employee`].
pub fn to_employee<E>(
	timesheet: &Timesheet,
	store: &Store,
) -> Result<Employee, <E as EmployeeAdapter>::Error>
where
	E: EmployeeAdapter,
{
	E::retrieve(
		&query::Employee {
			id: query::Match::EqualTo(Borrowed(&timesheet.employee_id)),
			..Default::default()
		},
		store,
	)?
	.into_iter()
	.next()
	.ok_or_else(|| Error::DataIntegrity(timesheet.employee_id).into())
}
