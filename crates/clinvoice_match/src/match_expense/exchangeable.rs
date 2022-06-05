use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::MatchExpense;

impl Exchangeable for MatchExpense
{
	/// # Summary
	///
	/// Exchange a the `cost` to another `currency`.
	fn exchange(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			id: self.id.clone(),
			category: self.category.clone(),
			cost: self.cost.exchange(currency, rates),
			description: self.description.clone(),
		}
	}
}
