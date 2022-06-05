use clinvoice_finance::{Exchangeable, ExchangeRates, Currency};

use super::MatchInvoice;

impl Exchangeable for MatchInvoice
{
	/// # Summary
	///
	/// Exchange a the `cost` to another `currency`.
	fn exchange(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			date_issued: self.date_issued.clone(),
			date_paid: self.date_paid.clone(),
			hourly_rate: self.hourly_rate.exchange(currency, rates),
		}
	}
}
