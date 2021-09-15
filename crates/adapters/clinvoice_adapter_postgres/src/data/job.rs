mod deletable;
mod job_adapter;
mod updatable;

clinvoice_adapter::AdaptJob!(PostgresJob<'job, sqlx::PgPool>);
