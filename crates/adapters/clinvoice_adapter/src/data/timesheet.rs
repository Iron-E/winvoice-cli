use
{
	std::borrow::Cow::Borrowed,

	super::{EmployeeAdapter, Error},
	crate::Store,

	clinvoice_data::{Employee, Timesheet},
	clinvoice_query as query,

	futures::FutureExt,
};

/// # Summary
///
/// Convert some `timesheet` into its referenced [`Employee`].
pub async fn to_employee<E>(timesheet: &Timesheet, store: &Store)
	-> Result<Employee, <E as EmployeeAdapter>::Error>
where
	E : EmployeeAdapter,
{
	let query = query::Employee
	{
		id: query::Match::EqualTo(Borrowed(&timesheet.employee_id)),
		..Default::default()
	};

	E::retrieve(&query, store).map(|result| result.and_then(|retrieved|
		retrieved.into_iter().next().ok_or_else(|| Error::DataIntegrity(timesheet.employee_id).into())
	)).await
}
