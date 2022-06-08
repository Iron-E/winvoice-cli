use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::Expense;

impl Exchangeable for Expense
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			cost: self.cost.exchange(currency, rates),
			..self
		}
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			category: self.category.clone(),
			cost: self.cost.exchange_ref(currency, rates),
			description: self.description.clone(),
			id: self.id,
			timesheet_id: self.timesheet_id,
		}
	}
}
