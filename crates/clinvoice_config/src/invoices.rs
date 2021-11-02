mod default;

use clinvoice_schema::Currency;
use serde::{Deserialize, Serialize};

/// # Summary
///
/// Configurations for [`Invoice`](clinvoice_schema::invoice::Invoice)s.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Invoices
{
	pub default_currency: Currency,
}
