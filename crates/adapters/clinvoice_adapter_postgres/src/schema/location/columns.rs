pub(in crate::schema) struct PgLocationColumns<'col>
{
	pub id: &'col str,
	pub outer_id: &'col str,
	pub name: &'col str,
}

impl PgLocationColumns<'static>
{
	pub(in crate::schema) const fn new() -> Self
	{
		Self {
			id: "id",
			outer_id: "outer_id",
			name: "name",
		}
	}
}
