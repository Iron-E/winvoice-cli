use core::str::FromStr;

use super::Currency;
use crate::{
	Error,
	Result,
};

impl FromStr for Currency
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		let uppercase = s.to_ascii_uppercase();
		Ok(match uppercase.as_str()
		{
			"AUD" => Self::AUD,
			"BGN" => Self::BGN,
			"BRL" => Self::BRL,
			"CAD" => Self::CAD,
			"CHF" => Self::CHF,
			"CNY" => Self::CNY,
			"CZK" => Self::CZK,
			"DKK" => Self::DKK,
			"EUR" => Self::EUR,
			"GBP" => Self::GBP,
			"HKD" => Self::HKD,
			"HRK" => Self::HRK,
			"HUF" => Self::HUF,
			"IDR" => Self::IDR,
			"ILS" => Self::ILS,
			"INR" => Self::INR,
			"ISK" => Self::ISK,
			"JPY" => Self::JPY,
			"KRW" => Self::KRW,
			"MXN" => Self::MXN,
			"MYR" => Self::MYR,
			"NOK" => Self::NOK,
			"NZD" => Self::NZD,
			"PHP" => Self::PHP,
			"PLN" => Self::PLN,
			"RON" => Self::RON,
			"RUB" => Self::RUB,
			"SEK" => Self::SEK,
			"SGD" => Self::SGD,
			"THB" => Self::THB,
			"TRY" => Self::TRY,
			"USD" => Self::USD,
			"ZAR" => Self::ZAR,
			_ => return Err(Error::UnsupportedCurrency(uppercase)),
		})
	}
}
