use
{
	crate::data::MatchWhen,
	clinvoice_data::{Contact, EmployeeStatus, Id},
};

pub struct Employee
{
	pub contact_info: MatchWhen<Contact>,
	pub id: MatchWhen<Id>,
	pub organization: MatchWhen<Id>,
	pub person: MatchWhen<Id>,
	pub title: MatchWhen<String>,
	pub status: MatchWhen<EmployeeStatus>,
}
