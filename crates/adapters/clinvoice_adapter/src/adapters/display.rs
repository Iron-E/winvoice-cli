use super::Adapters;

use core::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Adapters
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult
	{
		write!(formatter, "{}", match self {
			Adapters::TOML => "TOML"
		})
	}
}
