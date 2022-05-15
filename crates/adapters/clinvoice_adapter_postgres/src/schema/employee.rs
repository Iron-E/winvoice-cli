pub(super) mod columns;
mod deletable;
mod employee_adapter;
mod updatable;

use core::fmt::Write;
use std::collections::HashMap;

use clinvoice_schema::{Contact, Id};
use sqlx::{Executor, Postgres, Result};

/// Implementor of the [`EmployeeAdapter`](clinvoice_adapter::schema::EmployeeAdapter) for the
/// Postgres database.
pub struct PgEmployee;
