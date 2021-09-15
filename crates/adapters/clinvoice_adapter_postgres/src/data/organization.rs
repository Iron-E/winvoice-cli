mod deletable;
mod organization_adapter;
mod updatable;

clinvoice_adapter::AdaptOrganization!(PostgresOrganization<'org, sqlx::PgPool>);
