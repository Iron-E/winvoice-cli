mod display;

use clinvoice_schema::chrono::NaiveDateTime;

pub(crate) struct PostgresDateTime(pub(crate) NaiveDateTime);
