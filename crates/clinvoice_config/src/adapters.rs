mod display;

use serde::{Deserialize, Serialize};

/// File systems / DBMS which have been adapted to CLInvoice.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Adapters
{
	/// [PostgresSQL](https://www.postgresql.org/).
	Postgres,
}
