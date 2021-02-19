use
{
	super::OrganizationView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for OrganizationView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return writeln!(formatter, "{:?}", self);
	}
}
