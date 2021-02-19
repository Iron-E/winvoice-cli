use
{
	super::PersonView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for PersonView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return writeln!(formatter, "{:?}", self);
	}
}

