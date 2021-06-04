mod index;
mod try_from_path;

use
{
	std::{convert::TryInto, collections::HashMap, io, env, path::PathBuf},

	crate::Currency,

	chrono::{Datelike, Local},
	rust_decimal::Decimal,
};

pub(crate) struct ExchangeRates(HashMap<Currency, Decimal>);

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
	/// Returns `true` if there are locally-stored exchange rates that are from the past day.
	fn is_missing_or_outdated() -> io::Result<bool>
	{
		todo!(
"1. Read temporary storage for file
2. If file is found, continue. Else, return `false`
3. If date is correct, return `true`. Else, `false`"
		)
	}

	/// # Summary
	///
	/// Create a new [`ExchangeRates`] instance, which uses the [European Central Bank][ecb] to
	/// determine how to convert between currencies.
	///
	/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
	pub fn new() -> io::Result<Self>
	{
		if Self::is_missing_or_outdated()?
		{
			Self::scrape();
		}

		Self::filepath().as_path().try_into()
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
