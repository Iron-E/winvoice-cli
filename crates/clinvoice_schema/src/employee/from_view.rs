use super::Employee;
use crate::views::EmployeeView as View;

impl From<View> for Employee
{
	fn from(view: View) -> Self
	{
		Self {
			contact_info: view
				.contact_info
				.into_iter()
				.map(|(label, contact)| (label, contact.into()))
				.collect(),
			id: view.id,
			organization_id: view.organization.id,
			person_id: view.person.id,
			status: view.status,
			title: view.title,
		}
	}
}

impl From<&View> for Employee
{
	fn from(view: &View) -> Self
	{
		Self {
			contact_info: view
				.contact_info
				.clone()
				.into_iter()
				.map(|(label, contact)| (label, contact.into()))
				.collect(),
			id: view.id,
			organization_id: view.organization.id,
			person_id: view.person.id,
			status: view.status.clone(),
			title: view.title.clone(),
		}
	}
}
