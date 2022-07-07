mod default;

use clinvoice_schema::Currency;
use serde::{Deserialize, Serialize};

/// Configurations for [`Invoice`](clinvoice_schema::Invoice)s.
///
/// # Examples
///
/// ## TOML
///
/// ```rust
/// # assert!(toml::from_str::<clinvoice_config::Invoices>(r#"
/// default_currency = "USD"
/// # "#).is_ok());
/// ```
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Invoices
{
	/// The default currency should be used for the `hourly_rate` of an
	/// [`Invoice`](clinvoice_schema::Invoice).
	pub default_currency: Currency,
}
