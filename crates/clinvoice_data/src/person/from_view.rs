use
{
	super::Person,
	crate::views::PersonView as View,
};

impl From<View> for Person
{
	fn from(view: View) -> Self
	{
		Self
		{
			contact_info: view.contact_info.into_iter().map(|(label, contact)| (label, contact.into())).collect(),
			id: view.id,
			name: view.name,
		}
	}
}

impl From<&View> for Person
{
	fn from(view: &View) -> Self
	{
		Self
		{
			contact_info: view.contact_info.clone().into_iter().map(|(label, contact)| (label, contact.into())).collect(),
			id: view.id,
			name: view.name.clone(),
		}
	}
}
