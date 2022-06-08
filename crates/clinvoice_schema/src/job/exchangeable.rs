use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::Job;

impl Exchangeable for Job
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			invoice: self.invoice.exchange(currency, rates),
			..self
		}
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			client: self.client.clone(),
			date_close: self.date_close,
			date_open: self.date_open,
			id: self.id,
			increment: self.increment,
			invoice: self.invoice.exchange_ref(currency, rates),
			notes: self.notes.clone(),
			objectives: self.objectives.clone(),
		}
	}
}
