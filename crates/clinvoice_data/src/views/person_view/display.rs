use
{
	super::PersonView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for PersonView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		let mut sorted_contact_info = self.contact_info.clone();
		sorted_contact_info.sort();

		writeln!(formatter, "Contact Info:")?;
		sorted_contact_info.iter().try_for_each(|c| writeln!(formatter, "\t- {}", c))?;

		return write!(formatter, "\nName: {}", self.name);
	}
}

