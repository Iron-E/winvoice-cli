use super::TomlLocation;

use clinvoice_data::Location;

impl<'name> From<Location<'name>> for TomlLocation<'name>
{
	fn from(location: Location<'name>) -> Self
	{
		return TomlLocation (location);
	}
}
