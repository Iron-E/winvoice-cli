use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::ExpenseColumns;
use crate::fmt::ColumnsToSql;

impl<T> ColumnsToSql for ExpenseColumns<T>
where
	T: Copy + Display,
{
	fn push<Db>(&self, query: &mut QueryBuilder<Db>)
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

	fn push_set<Db>(&self, query: &mut QueryBuilder<Db>, values_ident: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scoped(values_ident);
		query
			.separated('=')
			.push(self.category)
			.push(values_columns.category)
			.push_unseparated(',')
			.push_unseparated(self.cost)
			.push(values_columns.cost)
			.push_unseparated(',')
			.push_unseparated(self.description)
			.push(values_columns.description)
			.push_unseparated(',')
			.push_unseparated(self.timesheet_id)
			.push(values_columns.timesheet_id);
	}

	fn push_unique<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		const UNIQUE: ExpenseColumns<&str> = ExpenseColumns::unique();
		query
			.separated(' ')
			.push(self.category)
			.push("AS")
			.push(UNIQUE.category)
			.push_unseparated(',')
			.push_unseparated(self.cost)
			.push("AS")
			.push(UNIQUE.cost)
			.push_unseparated(',')
			.push_unseparated(self.description)
			.push("AS")
			.push(UNIQUE.description)
			.push_unseparated(',')
			.push_unseparated(self.id)
			.push("AS")
			.push(UNIQUE.id)
			.push_unseparated(',')
			.push_unseparated(self.timesheet_id)
			.push("AS")
			.push(UNIQUE.timesheet_id);
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
