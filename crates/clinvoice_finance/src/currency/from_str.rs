use core::str::FromStr;

use super::Currency;
use crate::{Error, Result};

impl FromStr for Currency
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		let uppercase = s.to_ascii_uppercase();
		Ok(match uppercase.as_str()
		{
			"AUD" => Self::Aud,
			"BGN" => Self::Bgn,
			"BRL" => Self::Brl,
			"CAD" => Self::Cad,
			"CHF" => Self::Chf,
			"CNY" => Self::Cny,
			"CZK" => Self::Czk,
			"DKK" => Self::Dkk,
			"EUR" => Self::Eur,
			"GBP" => Self::Gbp,
			"HKD" => Self::Hkd,
			"HRK" => Self::Hrk,
			"HUF" => Self::Huf,
			"IDR" => Self::Idr,
			"ILS" => Self::Ils,
			"INR" => Self::Inr,
			"ISK" => Self::Isk,
			"JPY" => Self::Jpy,
			"KRW" => Self::Krw,
			"MXN" => Self::Mxn,
			"MYR" => Self::Myr,
			"NOK" => Self::Nok,
			"NZD" => Self::Nzd,
			"PHP" => Self::Php,
			"PLN" => Self::Pln,
			"RON" => Self::Ron,
			"RUB" => Self::Rub,
			"SEK" => Self::Sek,
			"SGD" => Self::Sgd,
			"THB" => Self::Thb,
			"TRY" => Self::Try,
			"USD" => Self::Usd,
			"ZAR" => Self::Zar,
			_ => return Err(Error::UnsupportedCurrency(uppercase)),
		})
	}
}
