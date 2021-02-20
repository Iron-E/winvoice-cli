use
{
	super::Organization,
	crate::views::OrganizationView as View,
};

impl From<View> for Organization
{
	fn from(view: View) -> Self
	{
		return Self
		{
			id: view.id,
			location_id: view.location.id,
			name: view.name,
		};
	}
}
