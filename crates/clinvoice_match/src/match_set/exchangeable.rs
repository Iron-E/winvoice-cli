use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::MatchSet;

impl<T> Exchangeable for MatchSet<T>
where
	T: Exchangeable,
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		self.map(|e| e.exchange(currency, rates))
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		self.map_ref(|e| e.exchange_ref(currency, rates))
	}
}
