use
{
	super::Location,
	crate::views::LocationView as View,
};

impl From<View> for Location
{
	fn from(view: View) -> Self
	{
		Self
		{
			id: view.id,
			outer_id: match view.outer
			{
				Some(location) => Some(location.id),
				_ => None,
			},
			name: view.name,
		}
	}
}

impl From<&View> for Location
{
	fn from(view: &View) -> Self
	{
		Self
		{
			id: view.id,
			outer_id: match &view.outer
			{
				Some(location) => Some(location.id),
				_ => None,
			},
			name: view.name.clone(),
		}
	}
}
