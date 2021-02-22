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
			Adapters::Bincode => "Bincode"
		})
	}
}

#[cfg(test)]
mod tests
{
	use super::Adapters;

	#[test]
	fn test_display()
	{
		assert_eq!(format!("{}", Adapters::Bincode), "Bincode");
	}
}
