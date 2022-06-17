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
	pub(super) fn row_to_view(columns: EmployeeColumns<&str>, row: &PgRow) -> Employee
	{
		Employee {
			id: row.get(columns.id),
			name: row.get(columns.name),
			status: row.get(columns.status),
			title: row.get(columns.title),
		}
	}
}
