use std::borrow::Cow::Borrowed;

use clinvoice_data::{Employee, Timesheet};
use clinvoice_query as query;
use futures::FutureExt;

use super::{EmployeeAdapter, Error};
use crate::Store;

/// # Summary
///
/// Convert some `timesheet` into its referenced [`Employee`].
pub async fn to_employee<E>(
	timesheet: &Timesheet,
	store: &Store,
) -> Result<Employee, <E as EmployeeAdapter>::Error>
where
	E: EmployeeAdapter,
{
	let query = query::Employee {
		id: query::Match::EqualTo(Borrowed(&timesheet.employee_id)),
		..Default::default()
	};

	E::retrieve(&query, store)
		.map(|result| {
			result.and_then(|retrieved| {
				retrieved
					.into_iter()
					.next()
					.ok_or_else(|| Error::DataIntegrity(timesheet.employee_id).into())
			})
		})
		.await
}
