mod from_str;
mod index;

use std::{
	collections::HashMap,
	convert::TryInto,
	env,
	fs,
	io::{
		Cursor,
		Error as IoError,
		ErrorKind::{InvalidData, NotFound, Unsupported},
		Read,
	},
	path::{Path, PathBuf},
};

use chrono::{Datelike, Local};
use futures::TryFutureExt;
use reqwest::Response;
use rust_decimal::Decimal;
use zip::{result::ZipError, ZipArchive};

use crate::{Currency, Error, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExchangeRates(HashMap<Currency, Decimal>);

impl ExchangeRates
{
	/// # Summary
	///
	/// Get the latest [`ExchangeRates`] from the ECB.
	async fn download(filepath: &Path) -> Result<String>
	{
		let cursor = reqwest::get("https://www.ecb.europa.eu/stats/eurofxref/eurofxref.zip")
			.and_then(Response::bytes)
			.map_ok(Cursor::new)
			.await?;

		fn zip_err_to_io(e: ZipError) -> IoError
		{
			match e
			{
				ZipError::Io(e2) => e2,
				ZipError::FileNotFound => NotFound.into(),
				ZipError::InvalidArchive(_) => InvalidData.into(),
				ZipError::UnsupportedArchive(_) => Unsupported.into(),
			}
		}

		let mut archive = ZipArchive::new(cursor).map_err(zip_err_to_io)?;
		let mut file = archive.by_index(0).map_err(zip_err_to_io)?;

		let mut csv = String::with_capacity(file.size().try_into().unwrap_or_default());
		file.read_to_string(&mut csv)?;

		fs::write(filepath, &csv)?;
		Ok(csv)
	}

	/// # Summary
	///
	/// Return the filepath which the latest exchange rates should be stored at.
	fn filepath() -> PathBuf
	{
		let today = Local::now();
		env::temp_dir().join(format!(
			"clinvoice_finance--{}-{}-{}.csv",
			today.year(),
			today.month(),
			today.day()
		))
	}

	/// # Summary
	///
	/// Retrieve the corresponding exchange rate for the `currency` provided.
	///
	/// # Returns
	///
	/// * [`None`] if this set of exchange rates does not account for the `currency`.
	/// * [`Some`] if this set of exchange rates accounts for the `currency`.
	pub fn get(&self, currency: Currency) -> Option<&Decimal>
	{
		self.0.get(&currency)
	}

	/// # Summary
	///
	/// Create a new [`ExchangeRates`] instance, which uses the [European Central Bank][ecb] to
	/// determine how to convert between currencies.
	///
	/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
	pub async fn new() -> Result<Self>
	{
		let filepath = Self::filepath();
		let contents = if filepath.is_file()
		{
			fs::read_to_string(&filepath)?
		}
		else
		{
			Self::download(&filepath).await?
		};

		contents.parse()
	}
}

#[cfg(test)]
mod tests
{
	use rust_decimal::Decimal;

	use super::{env, fs, ExchangeRates};
	use crate::{Currency, Error, SAMPLE_EXCHANGE_RATES_CSV};

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn download()
	{
		let filepath = env::temp_dir()
			.join("clinvoice_finance")
			.join("exchange-rates")
			.join("download.csv");

		if filepath.is_file()
		{
			fs::remove_file(&filepath).unwrap();
		}
		else
		{
			let parent = filepath.parent().unwrap();
			if !parent.is_dir()
			{
				fs::create_dir_all(parent).unwrap();
			}
		}

		ExchangeRates::download(&filepath).await.unwrap();

		assert!(filepath.is_file());
		assert!(fs::read_to_string(&filepath)
			.map_err(Error::from)
			.and_then(|s| s.parse::<ExchangeRates>())
			.is_ok());
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
	async fn new()
	{
		// WARN: if it becomes a new day while this test is running, there is a chance that
		//      `ExchangeRates::new` will not use this  because it is from a previous day.
		//      This will cause the test to fail, but indicates that `ExchangeRates` is working
		//      correctly.
		let filepath = ExchangeRates::filepath();

		fs::write(&filepath, SAMPLE_EXCHANGE_RATES_CSV).unwrap();

		// `ExchangeRates::new` will read existing data if it exists
		assert_eq!(
			ExchangeRates::new().await.unwrap(),
			ExchangeRates(
				[
					(Currency::AUD, Decimal::new(1_5792, 4)),
					(Currency::BGN, Decimal::new(1_9558, 4)),
					(Currency::BRL, Decimal::new(6_1894, 4)),
					(Currency::CAD, Decimal::new(1_4710, 4)),
					(Currency::CHF, Decimal::new(1_0961, 4)),
					(Currency::CNY, Decimal::new(7_7910, 4)),
					(Currency::CZK, Decimal::new(25_448, 3)),
					(Currency::DKK, Decimal::new(7_4365, 4)),
					(Currency::EUR, Decimal::new(1, 0)),
					(Currency::GBP, Decimal::new(85955, 5)),
					(Currency::HKD, Decimal::new(9_4551, 4)),
					(Currency::HRK, Decimal::new(7_5013, 4)),
					(Currency::HUF, Decimal::new(345_82, 2)),
					(Currency::IDR, Decimal::new(17420_91, 2)),
					(Currency::ILS, Decimal::new(3_9598, 4)),
					(Currency::INR, Decimal::new(8_88755, 4)),
					(Currency::ISK, Decimal::new(146_30, 2)),
					(Currency::JPY, Decimal::new(133_81, 2)),
					(Currency::KRW, Decimal::new(1357_75, 2)),
					(Currency::MXN, Decimal::new(24_3300, 4)),
					(Currency::MYR, Decimal::new(5_0241, 4)),
					(Currency::NOK, Decimal::new(10_1501, 4)),
					(Currency::NZD, Decimal::new(1_6915, 4)),
					(Currency::PHP, Decimal::new(58_208, 3)),
					(Currency::PLN, Decimal::new(4_4520, 4)),
					(Currency::RON, Decimal::new(4_9220, 4)),
					(Currency::RUB, Decimal::new(89_2163, 4)),
					(Currency::SEK, Decimal::new(10_1145, 4)),
					(Currency::SGD, Decimal::new(1_6141, 4)),
					(Currency::THB, Decimal::new(37_938, 3)),
					(Currency::TRY, Decimal::new(10_5650, 4)),
					(Currency::USD, Decimal::new(1_2187, 4)),
					(Currency::ZAR, Decimal::new(16_5218, 4)),
				]
				.into_iter()
				.collect(),
			),
		);

		// The method will download new data if it is missing.
		fs::remove_file(&filepath).unwrap();
		assert!(!filepath.is_file());
		ExchangeRates::new().await.unwrap();
		assert!(filepath.is_file());
	}
}
