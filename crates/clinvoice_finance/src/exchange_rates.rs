mod from_str;

use std::{
	collections::HashMap,
	convert::TryInto,
	env,
	fs,
	io::{Cursor, Read},
	ops::Range,
	path::PathBuf,
};

use chrono::{Datelike, Local};
use futures::TryFutureExt;
use reqwest::Response;
use rust_decimal::Decimal;
use zip::ZipArchive;

use crate::{Currency, Result};

/// A collection of rates of exchange between currencies such that some `amount` of [`Money`]
/// divided by its [`Currency`] will yield [`Currency::Eur`], and an `amount` of [`Currency::Eur`]
/// multiplied by any [`Currency`]'s exchange rate will yield that [`Currency`].
///
/// # See also
///
/// * [`ExchangeRates::get`], to get the corresponding rate for some [`Currency`].
/// * [`ExchangeRates::new`], to create new [`ExchangeRate`]s.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExchangeRates(HashMap<Currency, Decimal>);

impl ExchangeRates
{
	/// Return the [filepath](PathBuf) which the latest [`ExchangeRates`] should be stored at.
	///
	/// There will be a new filepath each day.
	fn filepath() -> PathBuf
	{
		let today = Local::now();
		env::temp_dir().join(format!(
			"clinvoice-finance--{}-{}-{}.csv",
			today.year(),
			today.month(),
			today.day()
		))
	}

	/// Retrieve the exchange rate for the `currency` provided, such that any [`Decimal`]
	/// [multiplied by](std::ops::Mul) the return value will convert it to the `desired`
	/// [`Currency`].
	///
	/// # Returns
	///
	/// * [`None`] if this set of exchange rates does not account for the `currency`.
	/// * [`Some`] if this set of exchange rates accounts for the `currency`.
	pub fn get(&self, current: &Currency, desired: &Currency) -> Option<Decimal>
	{
		self
			.0
			.get(current)
			.and_then(|c| self.0.get(desired).map(|d| d / c))
	}

	/// Same as [`ExchangeRates::get`], except using range syntax (i.e. `current..desired`) and
	/// panics with a custom error message instead of returning [`None`].
	///
	/// # Panics
	///
	/// * If any `Currency` in `range` is not present in this set of [`ExchangeRates`].
	pub fn index(&self, range: Range<&Currency>) -> Decimal
	{
		self.get(range.start, range.end).unwrap_or_else(|| {
			panic!(
				"Either {} or {} was not found in {self:?}",
				range.start, range.end
			)
		})
	}

	/// Create a new [`ExchangeRates`] instance, which uses the [European Central Bank][ecb] to
	/// determine how to convert between currencies.
	///
	/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
	pub async fn new() -> Result<Self>
	{
		let filepath = Self::filepath();

		// PERF: CLInvoice caches ECB data until `Self::filepath()` changes
		let contents = if filepath.is_file()
		{
			fs::read_to_string(&filepath)?
		}
		else
		{
			let cursor = reqwest::get("https://www.ecb.europa.eu/stats/eurofxref/eurofxref.zip")
				.and_then(Response::bytes)
				.map_ok(Cursor::new)
				.await?;

			let mut archive = ZipArchive::new(cursor)?;
			let mut csv = archive.by_index(0)?;

			let mut csv_contents = String::with_capacity(csv.size().try_into().unwrap());
			csv.read_to_string(&mut csv_contents)?;

			// cache the download for next time this method is called
			debug_assert!(
				!filepath.is_file(),
				"attemped to initialize `ExchangeRates` cache at {filepath:?}, but it already exists"
			);
			fs::write(filepath, &csv_contents)?;

			csv_contents
		};

		contents.parse()
	}
}
