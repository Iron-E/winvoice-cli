use
{
	super::Employee,
	crate::views::EmployeeView as View,
};

impl From<View> for Employee
{
	fn from(view: View) -> Self
	{
		return Self
		{
			contact_info: view.contact_info.into_iter().map(|c| c.into()).collect(),
			id: view.id,
			organization_id: view.organization.id,
			person_id: view.person.id,
			status: view.status,
			title: view.title,
		};
	}
}

impl From<&View> for Employee
{
	fn from(view: &View) -> Self
	{
		return Self
		{
			contact_info: view.contact_info.iter().cloned().map(|c| c.into()).collect(),
			id: view.id,
			organization_id: view.organization.id,
			person_id: view.person.id,
			status: view.status,
			title: view.title.clone(),
		};
	}
}
