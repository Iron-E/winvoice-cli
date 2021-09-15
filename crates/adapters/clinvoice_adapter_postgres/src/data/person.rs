mod deletable;
mod person_adapter;
mod updatable;

clinvoice_adapter::AdaptPerson!(PostgresPerson<'per, sqlx::PgPool>);
