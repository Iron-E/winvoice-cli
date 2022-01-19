use clinvoice_schema::Person;
use sqlx::{postgres::PgRow, Row};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::schema) struct PgPersonColumns<'col>
{
	pub id: &'col str,
	pub name: &'col str,
}

impl PgPersonColumns<'_>
{
	pub(in crate::schema) fn row_to_view(self, row: &PgRow) -> Person
	{
		Person {
			id: row.get(self.id),
			name: row.get(self.name),
		}
	}
}
