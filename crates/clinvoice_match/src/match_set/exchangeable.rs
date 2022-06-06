use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::MatchSet;

impl<T> Exchangeable for MatchSet<T>
where
	T: Exchangeable,
{
	/// # Summary
	///
	/// Exchange a the [`MatchExpense`]'s `cost` to another `currency`.
	fn exchange(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		self.map_ref(&|e| e.exchange(currency, rates))
	}
}
