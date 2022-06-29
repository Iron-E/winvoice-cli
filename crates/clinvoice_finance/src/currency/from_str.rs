use core::str::FromStr;

use super::Currency;
use crate::{Error, Result};

impl FromStr for Currency
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		[
			Currency::Aud,
			Currency::Bgn,
			Currency::Brl,
			Currency::Cad,
			Currency::Chf,
			Currency::Cny,
			Currency::Czk,
			Currency::Dkk,
			Currency::Eur,
			Currency::Gbp,
			Currency::Hkd,
			Currency::Hrk,
			Currency::Huf,
			Currency::Idr,
			Currency::Ils,
			Currency::Inr,
			Currency::Isk,
			Currency::Jpy,
			Currency::Krw,
			Currency::Mxn,
			Currency::Myr,
			Currency::Nok,
			Currency::Nzd,
			Currency::Php,
			Currency::Pln,
			Currency::Ron,
			Currency::Rub,
			Currency::Sek,
			Currency::Sgd,
			Currency::Thb,
			Currency::Try,
			Currency::Usd,
			Currency::Zar,
		]
		.into_iter()
		.find(|c| c.as_str().eq_ignore_ascii_case(s))
		.ok_or_else(|| Error::UnsupportedCurrency(s.to_string()))
	}
}
