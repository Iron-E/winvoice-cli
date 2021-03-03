use serde::{Deserialize, Serialize};

/// # Summary
///
/// Configurations for [`Invoice`](clinvoice_data::invoice::Invoice)s.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Invoices<'currency>
{
	pub default_currency: &'currency str,
}
