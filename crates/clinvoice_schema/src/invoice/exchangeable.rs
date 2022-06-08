use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::Invoice;

impl Exchangeable for Invoice
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			hourly_rate: self.hourly_rate.exchange(currency, rates),
			..self
		}
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			date: self.date,
			hourly_rate: self.hourly_rate.exchange_ref(currency, rates),
		}
	}
}
