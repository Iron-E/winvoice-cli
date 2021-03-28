use super::{Location, location_outer_default};

impl Default for Location<'_>
{
	fn default() -> Self
	{
		Self
		{
			outer: location_outer_default(),
			..Default::default()
		}
	}
}
