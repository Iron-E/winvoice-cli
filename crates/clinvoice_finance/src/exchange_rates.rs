mod index;
mod try_from_path;

use
{
	std::{convert::TryInto, collections::HashMap, env, path::PathBuf},

	crate::{Currency, Result},

	chrono::{Datelike, Local},
	rust_decimal::Decimal,
};

pub struct ExchangeRates(HashMap<Currency, Decimal>);

impl ExchangeRates
{
	/// # Summary
	///
	/// Return the filepath which the latest exchange rates should be stored at.
	fn filepath() -> PathBuf
	{
		let today = Local::now();
		env::temp_dir().join(format!("clinvoice_finance--{}-{}-{}", today.year(), today.month(), today.day()))
	}

	/// # Summary
	///
	/// Create a new [`ExchangeRates`] instance, which uses the [European Central Bank][ecb] to
	/// determine how to convert between currencies.
	///
	/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
	pub fn new() -> Result<Self>
	{
		let filepath = Self::filepath();
		if !filepath.is_file()
		{
			Self::scrape();
		}

		filepath.as_path().try_into()
	}

	/// # Summary
	///
	/// Get the latest [`ExchangeRates`] from the ECB.
	fn scrape()
	{
		todo!(
"1. Download ZIP file
2. Unzip file"
		)
	}

	/// # Summary
	///
	/// The URL which can be used to retrieve new exchange rates.
	const fn source_url() -> &'static str
	{
		"https://www.ecb.europa.eu/stats/eurofxref/eurofxref.zip"
	}
}

#[cfg(test)]
mod tests
{
	#[test]
	fn is_missing_or_outdated() {}

	#[test]
	fn new() {}

	#[test]
	fn scrape() {}
}
