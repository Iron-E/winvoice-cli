use
{
	super::{Organization, Person},
	crate::data::MatchWhen,
	clinvoice_data::{Contact, EmployeeStatus, Id},
};

/// # Summary
///
/// An [`Employee`](clinvoice_data::Employee) with [matchable](MatchWhen) fields.
pub struct Employee<'m>
{
	pub contact_info: MatchWhen<'m, Contact>,
	pub id: MatchWhen<'m, Id>,
	pub organization: Organization<'m>,
	pub person: Person<'m>,
	pub title: MatchWhen<'m, String>,
	pub status: MatchWhen<'m, EmployeeStatus>,
}
