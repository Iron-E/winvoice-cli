use
{
	super::{ContactView, EmployeeView},
	std::
	{
		collections::BTreeSet,
		fmt::{Display, Formatter, Result}
	},
};

impl Display for EmployeeView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		writeln!(formatter, "Contact Info:")?;
		let sorted_contact_info: BTreeSet<ContactView> = self.contact_info.iter().cloned().collect();
		sorted_contact_info.iter().try_for_each(|c| writeln!(formatter, "\t{}", c))?;

		writeln!(formatter, "\nOrganization:\n\t{}", self.organization.to_string().replace("\n", "\n\t"))?;
		writeln!(formatter, "\nPerson:\n\t{}", self.person.to_string().replace("\n", "\n\t"))?;
		writeln!(formatter, "\nStatus: {}", self.status)?;
		return writeln!(formatter, "\nTitle: {}", self.title);
	}
}

