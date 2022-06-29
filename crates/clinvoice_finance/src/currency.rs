mod display;
mod from_str;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// [ISO-4217][iso] currency codes which are reported by the [European Central Bank][ecb] for
/// exchange.
///
/// [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
/// [iso]: https://www.iso.org/iso-4217-currency-codes.html
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub enum Currency
{
	/// The Australian dollar.
	Aud,

	/// The Bulgarian lev.
	Bgn,

	/// The Brazilian real
	Brl,

	/// The Canadian dollar.
	Cad,

	/// The Swiss franc.
	Chf,

	/// The Chinese yuan.
	Cny,

	/// The Czech koruna.
	Czk,

	/// The Danish krone.
	Dkk,

	/// The Euro.
	#[default]
	Eur,

	/// The British pound.
	Gbp,

	/// The Hong Kong dollar.
	Hkd,

	/// The Croatian kuna.
	Hrk,

	/// The Hungarian forint.
	Huf,

	/// The Indonesian rupiah.
	Idr,

	/// The Israeli shekel.
	Ils,

	/// The Indian rupee.
	Inr,

	/// The Icelandic krona.
	Isk,

	/// The Japanese yen.
	Jpy,

	/// The South Korean won.
	Krw,

	/// The Mexican peso.
	Mxn,

	/// The Malaysian ringgit.
	Myr,

	/// The Norwegian krone.
	Nok,

	/// The New Zeland dollar.
	Nzd,

	/// The Philippine peso.
	Php,

	/// The Polish zloty.
	Pln,

	/// The Romanian leu.
	Ron,

	/// The Russian rouble.
	Rub,

	/// The Swedish krona.
	Sek,

	/// The Singapore dollar.
	Sgd,

	/// The Thai baht.
	Thb,

	/// The Turkish lira.
	Try,

	/// The US dollar.
	Usd,

	/// The South African rand.
	Zar,
}

impl Currency
{
	/// Retrieve a [`Currency`]'s string representation.
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

	/// The number of currencies supported by the program. Good for use when creating [`Vec`]s or [`HashMap`](std:collections::HashMap)s.
	pub const fn count() -> usize
	{
		// WARN: must be updated whenever the enum is changed.
		33
	}
}
