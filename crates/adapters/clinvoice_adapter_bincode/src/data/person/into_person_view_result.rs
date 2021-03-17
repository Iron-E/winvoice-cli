use
{
	super::BincodePerson,
	crate::data::Result,
	clinvoice_data::views::PersonView,
};

// NOTE: tests not needed because this is called in `Into<EmployeeView>`
impl Into<Result<PersonView>> for BincodePerson<'_, '_>
{
	fn into(self) -> Result<PersonView>
	{
		Ok(PersonView
		{
			id: self.person.id,
			name: self.person.name.clone(),
		})
	}
}
