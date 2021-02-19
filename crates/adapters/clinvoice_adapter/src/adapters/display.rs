use
{
	super::Adapters,
	core::fmt::{Display, Formatter, Result as FmtResult},
};

impl Display for Adapters
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult
	{
		write!(formatter, "{}", match self {
			Adapters::TOML => "TOML"
		})
	}
}
