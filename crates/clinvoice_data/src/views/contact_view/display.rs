use
{
	super::ContactView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for ContactView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return match self
		{
			ContactView::Address(location) => location.fmt(formatter),
			ContactView::Email(email) => writeln!(formatter, "{}", email),
			ContactView::Phone(phone) => writeln!(formatter, "{}", phone),
		};
	}
}
