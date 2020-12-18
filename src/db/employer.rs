use super::organization::Organization;

pub struct Employer<'location_name, 'location_outer, 'name> where 'location_outer : 'location_name
{
	pub organization: Organization<'location_name, 'location_outer, 'name>,
}
