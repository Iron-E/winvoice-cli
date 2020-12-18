use super::location::Location;

pub struct Organization<'location_name, 'location_outer, 'name> where 'location_outer : 'location_name
{
	pub location: Location<'location_name, 'location_outer>,
	pub name: &'name str,
}
