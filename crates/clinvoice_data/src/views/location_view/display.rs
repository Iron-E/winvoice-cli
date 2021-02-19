use
{
	super::LocationView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for LocationView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		let mut output = self.name.clone();
		let mut outer = &self.outer;

		while let Some(o) = outer
		{
			output.push_str(", ");
			output.push_str(&o.name);

			outer = &o.outer;
		}

		return writeln!(formatter, "{}", output);
	}
}
