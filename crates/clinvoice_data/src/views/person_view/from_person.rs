use
{
	super::PersonView,
	crate::Person,
};

impl From<Person> for PersonView
{
	fn from(person: Person) -> PersonView
	{
		PersonView
		{
			id: person.id,
			name: person.name,
		}
	}
}
