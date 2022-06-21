use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::ExpenseColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<T> ColumnsToSql for ExpenseColumns<T>
where
	T: Copy + Display,
{
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push(self.category)
			.push(self.cost)
			.push(self.description)
			.push(self.id)
			.push(self.timesheet_id);
	}

	fn push_set_to<Db>(&self, query: &mut QueryBuilder<Db>, values_alias: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scope(values_alias);
		query
			.push_equal(self.category, values_columns.category)
			.push(',')
			.push_equal(self.cost, values_columns.cost)
			.push(',')
			.push_equal(self.description, values_columns.description)
			.push(',')
			.push_equal(self.timesheet_id, values_columns.timesheet_id);
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
