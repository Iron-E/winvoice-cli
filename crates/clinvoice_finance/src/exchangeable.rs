use crate::{Currency, ExchangeRates};

/// # Summary
///
/// Implementors of this trait contain quantities which are relative to the `currency` they are
/// currently in. To view them in another currency, they must be [`Exchangeable::exchange`]d using
/// the `rates` of conversion.
pub trait Exchangeable
{
	/// # Summary
	///
	/// Exchange some quantity into another `currency` using `rates`.
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self;

	/// # Summary
	///
	/// Exchange some quantity into another `currency` using `rates`.
	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self;
}

impl<T> Exchangeable for Vec<T>
where
	T: Exchangeable,
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		self
			.into_iter()
			.map(|exchangeable| exchangeable.exchange(currency, rates))
			.collect()
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		self
			.iter()
			.map(|exchangeable| exchangeable.exchange_ref(currency, rates))
			.collect()
	}
}
