mod display;

use serde::{Deserialize, Serialize};

/// # Summary
///
/// Currently supported file systems / DBMS.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Adapters
{
	/// # Summary
	///
	/// A bincode filesystem.
	Postgres,
}
