use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::LocationColumns;
use crate::fmt::ColumnsToSql;

impl<T> ColumnsToSql for LocationColumns<T>
where
	T: Copy + Display,
{
	fn push<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push(self.id)
			.push(self.name)
			.push(self.outer_id);
	}

	fn push_set<Db>(&self, query: &mut QueryBuilder<Db>, values_alias: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scope(values_alias);
		query
			.separated('=')
			.push(self.name)
			.push(values_columns.name)
			.push_unseparated(',')
			.push_unseparated(self.outer_id)
			.push(values_columns.outer_id);
	}

	fn push_unique<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		const UNIQUE: LocationColumns<&str> = LocationColumns::unique();
		query
			.separated(' ')
			.push(self.id)
			.push("AS")
			.push(UNIQUE.id)
			.push_unseparated(',')
			.push_unseparated(self.outer_id)
			.push("AS")
			.push(UNIQUE.outer_id)
			.push_unseparated(',')
			.push_unseparated(self.name)
			.push("AS")
			.push(UNIQUE.name);
	}

	fn push_update_where<Db>(
		&self,
		query: &mut QueryBuilder<Db>,
		table_alias: impl Copy + Display,
		values_alias: impl Copy + Display,
	) where
		Db: Database,
	{
		query
			.separated('=')
			.push(self.scope(table_alias).id)
			.push(self.scope(values_alias).id);
	}
}
