use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::LocationColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<T> ColumnsToSql for LocationColumns<T>
where
	T: Copy + Display,
{
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push(self.id)
			.push(self.name)
			.push(self.outer_id);
	}

	fn push_set_to<Db>(&self, query: &mut QueryBuilder<Db>, values_alias: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scope(values_alias);
		query
			.push_equal(self.name, values_columns.name)
			.push(',')
			.push_equal(self.outer_id, values_columns.outer_id);
	}

	fn push_update_where_to<Db>(
		&self,
		query: &mut QueryBuilder<Db>,
		table_alias: impl Copy + Display,
		values_alias: impl Copy + Display,
	) where
		Db: Database,
	{
		query.push_equal(self.scope(table_alias).id, self.scope(values_alias).id);
	}
}
