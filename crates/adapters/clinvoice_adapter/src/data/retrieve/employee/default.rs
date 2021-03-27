use super::{Employee, contact_view_default};

impl Default for Employee<'_>
{
	fn default() -> Self
	{
		Self
		{
			contact_info: contact_view_default(),
			..Default::default()
		}
	}
}
