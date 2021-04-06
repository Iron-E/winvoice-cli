use
{
	std::borrow::Cow,

	super::{EmployeeAdapter, Error, Match, query},
	crate::Store,

	clinvoice_data::{Employee, Timesheet},
};

/// # Summary
///
/// Convert some `timesheet` into its referenced [`Employee`].
pub fn to_employee<'store, E>(timesheet: &Timesheet, store: &'store Store)
	-> Result<Employee, <E as EmployeeAdapter<'store>>::Error>
where
	E : EmployeeAdapter<'store>,
{
	match E::retrieve(
		query::Employee
		{
			id: Match::EqualTo(Cow::Borrowed(&timesheet.employee_id)),
			..Default::default()
		},
		store,
	)?.first()
	{
		Some(employee) => Ok(employee.clone()),
		_ => Err(Error::DataIntegrity(timesheet.employee_id).into()),
	}
}
