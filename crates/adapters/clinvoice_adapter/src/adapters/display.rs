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
	use
	{
		super::Adapters,
		std::time::Instant,
	};

	#[test]
	fn test_display()
	{
		let start = Instant::now();
		assert_eq!(format!("{}", Adapters::Bincode), "Bincode");
		println!("\n>>>>> Adapters test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
