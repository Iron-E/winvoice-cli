use clinvoice_schema::views::PersonView;
use sqlx::{postgres::PgRow, Row};

mod deletable;
mod person_adapter;
mod updatable;

pub struct PgPerson;

impl PgPerson
{
	pub(super) fn row_to_view(row: &PgRow, id: &str, name: &str) -> PersonView
	{
		PersonView {
			id: row.get(id),
			name: row.get(name),
		}
	}
}
