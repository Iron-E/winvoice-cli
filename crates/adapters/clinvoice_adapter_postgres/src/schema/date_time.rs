mod display;

use clinvoice_schema::chrono::NaiveDateTime;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PostgresDateTime(pub(crate) NaiveDateTime);
