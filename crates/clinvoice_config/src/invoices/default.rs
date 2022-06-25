use super::{Currency, Invoices};

impl Default for Invoices
{
	fn default() -> Self
	{
		Self {
			default_currency: Currency::Usd,
		}
	}
}
