mod index;
mod try_from_path;

use
{
	std::
	{
		convert::TryInto,
		collections::HashMap,
		env,
		fs,
		io::{Cursor, Read},
		path::{Path, PathBuf},
	},

	crate::{Currency, Result},

	chrono::{Datelike, Local},
	rust_decimal::Decimal,
	zip::ZipArchive,
};

pub struct ExchangeRates(HashMap<Currency, Decimal>);

impl ExchangeRates
{
	/// # Summary
	///
	/// Get the latest [`ExchangeRates`] from the ECB.
	fn download(filepath: &Path) -> Result<()>
	{
		let response = reqwest::blocking::get("https://www.ecb.europa.eu/stats/eurofxref/eurofxref.zip")?;
		let bytes = response.bytes()?;

		let mut archive = ZipArchive::new(Cursor::new(bytes))?;
		let mut file = archive.by_index(0)?;

		let mut csv = Vec::new();
		file.read_to_end(&mut csv)?;

		fs::write(filepath, csv).map_err(|e| e.into())
	}

	/// # Summary
	///
	/// Return the filepath which the latest exchange rates should be stored at.
	fn filepath() -> PathBuf
	{
		let today = Local::now();
		env::temp_dir().join(format!("clinvoice_finance--{}-{}-{}.csv", today.year(), today.month(), today.day()))
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
		if !filepath.is_file() { Self::download(&filepath)?; }

		filepath.as_path().try_into()
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{convert::TryFrom, time::Instant},
		super::{env, ExchangeRates, fs},
	};

	#[test]
	fn download()
	{
		let filepath = env::temp_dir().join("clinvoice_finance").join("exchange-rates").join("download.csv");

		if filepath.is_file() { fs::remove_file(&filepath).unwrap(); }

		let parent = filepath.parent().unwrap();
		if !parent.is_dir() { fs::create_dir_all(parent).unwrap(); }

		let start = Instant::now();
		ExchangeRates::download(&filepath).unwrap();
		println!("\n>>>>> ExchangeRates::download {}s <<<<<\n", Instant::now().duration_since(start).as_secs_f64());

		assert!(filepath.is_file());
		assert!(ExchangeRates::try_from(filepath.as_path()).is_ok());
	}

	#[test]
	fn new()
	{
		let filepath = ExchangeRates::filepath();
		if filepath.is_file() { fs::remove_file(&filepath).unwrap(); }

		let start = Instant::now();
		// First ::new downloads the file
		ExchangeRates::new().unwrap();
		// Second ::new reads it
		ExchangeRates::new().unwrap();
		println!("\n>>>>> ExchangeRates::new {}s <<<<<\n", Instant::now().duration_since(start).as_secs_f64() / 2.0);

		assert!(filepath.is_file());
	}
}
