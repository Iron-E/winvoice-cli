mod deletable;
mod location_adapter;
mod updatable;

clinvoice_adapter::AdaptLocation!(PostgresLocation<'loc, sqlx::PgPool>);
