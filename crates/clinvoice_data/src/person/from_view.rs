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
			contact_info: view.contact_info.into_iter().map(|c| c.into()).collect(),
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
			contact_info: view.contact_info.iter().cloned().map(|c| c.into()).collect(),
			id: view.id,
			name: view.name.clone(),
		}
	}
}
