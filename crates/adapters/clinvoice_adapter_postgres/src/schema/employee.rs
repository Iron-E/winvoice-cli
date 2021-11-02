mod deletable;
mod employee_adapter;
mod updatable;

/// Implementor of the [`EmployeeAdapter`](clinvoice_adapter::schema::EmployeeAdapter) for the
/// Postgres database.
pub struct PostgresEmployee;
