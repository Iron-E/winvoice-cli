use
{
	super::BincodePerson,
	crate::data::contact,
	clinvoice_adapter::DynamicResult,
	clinvoice_data::views::PersonView,
};

// NOTE: tests not needed because this is called in `Into<EmployeeView>`
impl Into<DynamicResult<PersonView>> for BincodePerson<'_, '_, '_>
{
	fn into(self) -> DynamicResult<PersonView>
	{
		return Ok(PersonView
		{
			contact_info: contact::into_views(self.person.contact_info, self.store)?,
			id: self.person.id,
			name: self.person.name,
		});
	}
}
