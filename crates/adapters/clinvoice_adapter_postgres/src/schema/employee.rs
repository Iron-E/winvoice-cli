use clinvoice_adapter::schema::columns::EmployeeColumns;
use clinvoice_schema::Employee;
use sqlx::{postgres::PgRow, Row};

mod deletable;
mod employee_adapter;
mod updatable;

/// Implementor of the [`EmployeeAdapter`](clinvoice_adapter::schema::EmployeeAdapter) for the
/// Postgres database.
pub struct PgEmployee;

impl PgEmployee
{
	pub(super) fn row_to_view<T>(columns: EmployeeColumns<T>, row: &PgRow) -> Employee
	where
		T: AsRef<str>,
	{
		Employee {
			id: row.get(columns.id.as_ref()),
			name: row.get(columns.name.as_ref()),
			status: row.get(columns.status.as_ref()),
			title: row.get(columns.title.as_ref()),
		}
	}
}
