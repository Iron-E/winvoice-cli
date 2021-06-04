use
{
	std::ops::Index,

	super::ExchangeRates,
	crate::Currency,

	rust_decimal::Decimal,
};

impl Index<Currency> for ExchangeRates
{
	type Output = Decimal;

	fn index(&self, index: Currency) -> &Self::Output
	{
		self.0.get(&index).expect(&format!("{} was not found in this set of exchange rates", index))
	}
}
