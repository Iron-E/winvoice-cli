use super::Location;
use crate::views::LocationView as View;

impl From<View> for Location
{
	fn from(view: View) -> Self
	{
		Self {
			id: view.id,
			outer_id: view.outer.map(|l| l.id),
			name: view.name,
		}
	}
}

impl From<&View> for Location
{
	fn from(view: &View) -> Self
	{
		Self {
			id: view.id,
			outer_id: view.outer.as_ref().map(|l| l.id),
			name: view.name.clone(),
		}
	}
}
