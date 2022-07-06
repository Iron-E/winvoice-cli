mod default;

use clinvoice_schema::Currency;
use serde::{Deserialize, Serialize};

/// Configurations for [`Invoice`](clinvoice_schema::Invoice)s.
///
/// # Examples
///
/// ```rust
/// use clinvoice_config::Invoices;
/// use clinvoice_schema::Currency;
///
/// let _ = Invoices {
///   default_currency: Currency::Usd,
/// };
/// ```
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Invoices
{
	/// The default currency should be used for the `hourly_rate` of an
	/// [`Invoice`](clinvoice_schema::Invoice).
	pub default_currency: Currency,
}
