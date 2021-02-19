use
{
	super::{ContactView, PersonView},
	std::
	{
		collections::BTreeSet,
		fmt::{Display, Formatter, Result}
	},
};

impl Display for PersonView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		writeln!(formatter, "Contact Info:")?;
		let sorted_contact_info: BTreeSet<ContactView> = self.contact_info.iter().cloned().collect();
		sorted_contact_info.iter().try_for_each(|c| writeln!(formatter, "\t{}", c))?;

		return writeln!(formatter, "\nName: {}", self.name);
	}
}

