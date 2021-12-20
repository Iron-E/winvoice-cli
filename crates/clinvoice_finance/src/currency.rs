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
	AUD,

	/// # Summary
	///
	/// The Bulgarian lev.
	BGN,

	/// # Summary
	///
	/// The Brazilian real
	BRL,

	/// # Summary
	///
	/// The Canadian dollar.
	CAD,

	/// # Summary
	///
	/// The Swiss franc.
	CHF,

	/// # Summary
	///
	/// The Chinese yuan.
	CNY,

	/// # Summary
	///
	/// The Czech koruna.
	CZK,

	/// # Summary
	///
	/// The Danish krone.
	DKK,

	/// # Summary
	///
	/// The Euro.
	EUR,

	/// # Summary
	///
	/// The British pound.
	GBP,

	/// # Summary
	///
	/// The Hong Kong dollar.
	HKD,

	/// # Summary
	///
	/// The Croatian kuna.
	HRK,

	/// # Summary
	///
	/// The Hungarian forint.
	HUF,

	/// # Summary
	///
	/// The Indonesian rupiah.
	IDR,

	/// # Summary
	///
	/// The Israeli shekel.
	ILS,

	/// # Summary
	///
	/// The Indian rupee.
	INR,

	/// # Summary
	///
	/// The Icelandic krona.
	ISK,

	/// # Summary
	///
	/// The Japanese yen.
	JPY,

	/// # Summary
	///
	/// The South Korean won.
	KRW,

	/// # Summary
	///
	/// The Mexican peso.
	MXN,

	/// # Summary
	///
	/// The Malaysian ringgit.
	MYR,

	/// # Summary
	///
	/// The Norwegian krone.
	NOK,

	/// # Summary
	///
	/// The New Zeland dollar.
	NZD,

	/// # Summary
	///
	/// The Philippine peso.
	PHP,

	/// # Summary
	///
	/// The Polish zloty.
	PLN,

	/// # Summary
	///
	/// The Romanian leu.
	RON,

	/// # Summary
	///
	/// The Russian rouble.
	RUB,

	/// # Summary
	///
	/// The Swedish krona.
	SEK,

	/// # Summary
	///
	/// The Singapore dollar.
	SGD,

	/// # Summary
	///
	/// The Thai baht.
	THB,

	/// # Summary
	///
	/// The Turkish lira.
	TRY,

	/// # Summary
	///
	/// The US dollar.
	USD,

	/// # Summary
	///
	/// The South African rand.
	ZAR,
}

impl Currency
{
	/// Foo
	pub const fn as_str(&self) -> &'static str
	{
		match self
		{
			Self::AUD => "AUD",
			Self::BGN => "BGN",
			Self::BRL => "BRL",
			Self::CAD => "CAD",
			Self::CHF => "CHF",
			Self::CNY => "CNY",
			Self::CZK => "CZK",
			Self::DKK => "DKK",
			Self::EUR => "EUR",
			Self::GBP => "GBP",
			Self::HKD => "HKD",
			Self::HRK => "HRK",
			Self::HUF => "HUF",
			Self::IDR => "IDR",
			Self::ILS => "ILS",
			Self::INR => "INR",
			Self::ISK => "ISK",
			Self::JPY => "JPY",
			Self::KRW => "KRW",
			Self::MXN => "MXN",
			Self::MYR => "MYR",
			Self::NOK => "NOK",
			Self::NZD => "NZD",
			Self::PHP => "PHP",
			Self::PLN => "PLN",
			Self::RON => "RON",
			Self::RUB => "RUB",
			Self::SEK => "SEK",
			Self::SGD => "SGD",
			Self::THB => "THB",
			Self::TRY => "TRY",
			Self::USD => "USD",
			Self::ZAR => "ZAR",
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
