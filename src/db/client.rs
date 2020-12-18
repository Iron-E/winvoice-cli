use super::organization::Organization;

pub struct Client<'location_name, 'location_outer, 'name> where 'location_name : 'location_outer
{
	pub organization: Organization<'location_name, 'location_outer, 'name>
}
