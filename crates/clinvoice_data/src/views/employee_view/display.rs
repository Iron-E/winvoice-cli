use
{
	super::EmployeeView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for EmployeeView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		let mut sorted_contact_info = self.contact_info.clone();
		sorted_contact_info.sort();

		writeln!(formatter, "Contact Info:")?;
		sorted_contact_info.iter().try_for_each(|c| writeln!(formatter, "\t- {}", c))?;

		writeln!(formatter, "\nOrganization:\n\t{}", self.organization.to_string().replace("\n", "\n\t"))?;
		writeln!(formatter, "\nPerson:\n\t{}", self.person.to_string().replace("\n", "\n\t"))?;
		writeln!(formatter, "\nStatus: {}", self.status)?;
		return write!(formatter, "\nTitle: {}", self.title);
	}
}

