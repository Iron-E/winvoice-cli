use
{
	core::fmt::{Display, Formatter, Result as FmtResult},

	super::Adapters,
};

impl Display for Adapters
{
	fn fmt(&self, formatter: &mut Formatter) -> FmtResult
	{
		write!(formatter, "{}", match self
		{
			Adapters::Bincode => "Bincode",
			Adapters::Postgres => "Postgres",
		})
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::time::Instant,

		super::Adapters,
	};

	#[test]
	fn display()
	{
		let start = Instant::now();
		assert_eq!(format!("{}", Adapters::Bincode), "Bincode");
		println!("\n>>>>> Adapters::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
