use super::PersonView;
use crate::Person;

impl From<Person> for PersonView
{
	fn from(person: Person) -> PersonView
	{
		PersonView {
			id:   person.id,
			name: person.name,
		}
	}
}

impl From<&Person> for PersonView
{
	fn from(person: &Person) -> PersonView
	{
		PersonView {
			id:   person.id,
			name: person.name.clone(),
		}
	}
}
