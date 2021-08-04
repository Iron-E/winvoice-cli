use super::Person;
use crate::views::PersonView as View;

impl From<View> for Person
{
	fn from(view: View) -> Self
	{
		Self {
			id:   view.id,
			name: view.name,
		}
	}
}

impl From<&View> for Person
{
	fn from(view: &View) -> Self
	{
		Self {
			id:   view.id,
			name: view.name.clone(),
		}
	}
}
