use
{
	super::{Contact, Employee},
	crate::views::EmployeeView as View,
};

impl From<View> for Employee
{
	fn from(view: View) -> Self
	{
		return Self
		{
			contact_info: view.contact_info.into_iter().map(|c| Contact::from(c)).collect(),
			id: view.id,
			organization_id: view.organization.id,
			person_id: view.person.id,
			status: view.status,
			title: view.title,
		};
	}
}
