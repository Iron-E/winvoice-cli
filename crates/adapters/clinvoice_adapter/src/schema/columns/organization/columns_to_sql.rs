use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::OrganizationColumns;
use crate::fmt::ColumnsToSql;

impl<T> ColumnsToSql for OrganizationColumns<T>
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
			.push(self.location_id)
			.push(self.name);
	}

	fn push_set<Db>(&self, query: &mut QueryBuilder<Db>, values_ident: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scoped(values_ident);
		query
			.separated('=')
			.push(self.location_id)
			.push(values_columns.location_id)
			.push_unseparated(',')
			.push_unseparated(self.name)
			.push(values_columns.name);
	}

	fn push_update_where<Db>(
		&self,
		query: &mut QueryBuilder<Db>,
		table_ident: impl Copy + Display,
		values_ident: impl Copy + Display,
	) where
		Db: Database,
	{
		query
			.separated('=')
			.push(self.scoped(table_ident).id)
			.push(self.scoped(values_ident).id);
	}
}
