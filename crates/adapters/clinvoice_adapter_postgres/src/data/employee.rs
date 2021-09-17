mod deletable;
mod employee_adapter;
mod updatable;

/// Implementor of the [`EmployeeAdapter`](clinvoice_adapter::data::EmployeeAdapter) for the
/// Postgres database.
pub struct PostgresEmployee;
