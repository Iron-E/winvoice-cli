mod display;

use serde::{Deserialize, Serialize};

/// File systems / DBMS which have been adapted to CLInvoice.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Adapters
{
	/// [PostgresSQL](https://www.postgresql.org/).
	Postgres,
}
