use
{
	super::BincodePerson,
	crate::data::{contact, Result},
	clinvoice_data::views::PersonView,
};

// NOTE: tests not needed because this is called in `Into<EmployeeView>`
impl Into<Result<PersonView>> for BincodePerson<'_>
{
	fn into(self) -> Result<PersonView>
	{
		Ok(PersonView
		{
			contact_info: contact::into_views(self.person.contact_info, self.store)?,
			id: self.person.id,
			name: self.person.name,
		})
	}
}
