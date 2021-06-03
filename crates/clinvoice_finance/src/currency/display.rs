use
{
	core::fmt::{Display, Formatter, Result},

	super::Currency,
};

impl Display for Currency
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		match self
		{
			Self::AUD => write!(formatter, "AUD"),
			Self::BGN => write!(formatter, "BGN"),
			Self::BRL => write!(formatter, "BRL"),
			Self::CAD => write!(formatter, "CAD"),
			Self::CHF => write!(formatter, "CHF"),
			Self::CNY => write!(formatter, "CNY"),
			Self::CZK => write!(formatter, "CZK"),
			Self::DKK => write!(formatter, "DKK"),
			Self::EUR => write!(formatter, "EUR"),
			Self::GBP => write!(formatter, "GBP"),
			Self::HKD => write!(formatter, "HKD"),
			Self::HRK => write!(formatter, "HRK"),
			Self::HUF => write!(formatter, "HUF"),
			Self::IDR => write!(formatter, "IDR"),
			Self::ILS => write!(formatter, "ILS"),
			Self::INR => write!(formatter, "INR"),
			Self::ISK => write!(formatter, "ISK"),
			Self::JPY => write!(formatter, "JPY"),
			Self::KRW => write!(formatter, "KRW"),
			Self::MXN => write!(formatter, "MXN"),
			Self::MYR => write!(formatter, "MYR"),
			Self::NOK => write!(formatter, "NOK"),
			Self::NZD => write!(formatter, "NZD"),
			Self::PHP => write!(formatter, "PHP"),
			Self::PLN => write!(formatter, "PLN"),
			Self::RON => write!(formatter, "RON"),
			Self::RUB => write!(formatter, "RUB"),
			Self::SEK => write!(formatter, "SEK"),
			Self::SGD => write!(formatter, "SGD"),
			Self::THB => write!(formatter, "THB"),
			Self::TRY => write!(formatter, "TRY"),
			Self::USD => write!(formatter, "USD"),
			Self::ZAR => write!(formatter, "ZAR"),
		}
	}
}
