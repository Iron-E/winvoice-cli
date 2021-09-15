mod deletable;
mod employee_adapter;
mod updatable;

clinvoice_adapter::AdaptEmployee!(PostgresEmployee<'emp, sqlx::PgPool>);
