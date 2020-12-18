use super::{employer::Employer, person::Person};

pub struct Employee<'employer_location, 'employer_location_outer, 'employer_name, 'person_name> where 'employer_location_outer : 'employer_location
{
	pub employer: Employer<'employer_location, 'employer_location_outer, 'employer_name>,
	pub person: Person<'person_name>,
}
