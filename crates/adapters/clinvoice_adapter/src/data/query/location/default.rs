use super::{Location, location_outer_default};

impl Default for Location<'_>
{
	fn default() -> Self
	{
		Self
		{
			id: Default::default(),
			name: Default::default(),
			outer: location_outer_default(),
		}
	}
}
