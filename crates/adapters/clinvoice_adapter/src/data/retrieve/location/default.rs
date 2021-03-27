use super::{Location, outer_default};

impl Default for Location<'_>
{
	fn default() -> Self
	{
		Self
		{
			outer: outer_default(),
			..Default::default()
		}
	}
}
