mod default;
mod display;
mod from_str;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// [ISO-4217][iso] currency codes which are reported by the [European Central Bank][ecb] for
/// exchange.
///
/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
/// [iso]: https://www.iso.org/iso-4217-currency-codes.html
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum Currency
{
	/// # Summary
	///
	/// The Australian dollar.
	Aud,

	/// # Summary
	///
	/// The Bulgarian lev.
	Bgn,

	/// # Summary
	///
	/// The Brazilian real
	Brl,

	/// # Summary
	///
	/// The Canadian dollar.
	Cad,

	/// # Summary
	///
	/// The Swiss franc.
	Chf,

	/// # Summary
	///
	/// The Chinese yuan.
	Cny,

	/// # Summary
	///
	/// The Czech koruna.
	Czk,

	/// # Summary
	///
	/// The Danish krone.
	Dkk,

	/// # Summary
	///
	/// The Euro.
	Eur,

	/// # Summary
	///
	/// The British pound.
	Gbp,

	/// # Summary
	///
	/// The Hong Kong dollar.
	Hkd,

	/// # Summary
	///
	/// The Croatian kuna.
	Hrk,

	/// # Summary
	///
	/// The Hungarian forint.
	Huf,

	/// # Summary
	///
	/// The Indonesian rupiah.
	Idr,

	/// # Summary
	///
	/// The Israeli shekel.
	Ils,

	/// # Summary
	///
	/// The Indian rupee.
	Inr,

	/// # Summary
	///
	/// The Icelandic krona.
	Isk,

	/// # Summary
	///
	/// The Japanese yen.
	Jpy,

	/// # Summary
	///
	/// The South Korean won.
	Krw,

	/// # Summary
	///
	/// The Mexican peso.
	Mxn,

	/// # Summary
	///
	/// The Malaysian ringgit.
	Myr,

	/// # Summary
	///
	/// The Norwegian krone.
	Nok,

	/// # Summary
	///
	/// The New Zeland dollar.
	Nzd,

	/// # Summary
	///
	/// The Philippine peso.
	Php,

	/// # Summary
	///
	/// The Polish zloty.
	Pln,

	/// # Summary
	///
	/// The Romanian leu.
	Ron,

	/// # Summary
	///
	/// The Russian rouble.
	Rub,

	/// # Summary
	///
	/// The Swedish krona.
	Sek,

	/// # Summary
	///
	/// The Singapore dollar.
	Sgd,

	/// # Summary
	///
	/// The Thai baht.
	Thb,

	/// # Summary
	///
	/// The Turkish lira.
	Try,

	/// # Summary
	///
	/// The US dollar.
	Usd,

	/// # Summary
	///
	/// The South African rand.
	Zar,
}

impl Currency
{
	/// Foo
	pub const fn as_str(&self) -> &'static str
	{
		match self
		{
			Self::Aud => "AUD",
			Self::Bgn => "BGN",
			Self::Brl => "BRL",
			Self::Cad => "CAD",
			Self::Chf => "CHF",
			Self::Cny => "CNY",
			Self::Czk => "CZK",
			Self::Dkk => "DKK",
			Self::Eur => "EUR",
			Self::Gbp => "GBP",
			Self::Hkd => "HKD",
			Self::Hrk => "HRK",
			Self::Huf => "HUF",
			Self::Idr => "IDR",
			Self::Ils => "ILS",
			Self::Inr => "INR",
			Self::Isk => "ISK",
			Self::Jpy => "JPY",
			Self::Krw => "KRW",
			Self::Mxn => "MXN",
			Self::Myr => "MYR",
			Self::Nok => "NOK",
			Self::Nzd => "NZD",
			Self::Php => "PHP",
			Self::Pln => "PLN",
			Self::Ron => "RON",
			Self::Rub => "RUB",
			Self::Sek => "SEK",
			Self::Sgd => "SGD",
			Self::Thb => "THB",
			Self::Try => "TRY",
			Self::Usd => "USD",
			Self::Zar => "ZAR",
		}
	}

	/// # Summary
	///
	/// The number of currencies supported by the program.
	///
	/// Good for use when creating [`Vec`]s or [`HashMap`](std:collections::HashMap)s.
	pub const fn count() -> usize
	{
		// WARN: must be updated whenever the enum is changed.
		33
	}
}
