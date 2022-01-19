use clinvoice_schema::views::OrganizationView;
use sqlx::{postgres::PgRow, Executor, Postgres, Result, Row};

use super::PgLocation;

pub(super) mod columns;
mod deletable;
mod organization_adapter;
mod updatable;

pub struct PgOrganization;
