use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::EmployeeColumns;
use crate::fmt::ColumnsToSql;

impl<T> ColumnsToSql for EmployeeColumns<T>
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
			.push(self.status)
			.push(self.title);
	}

	fn push_set<Db>(&self, query: &mut QueryBuilder<Db>, values_ident: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scoped(values_ident);
		query
			.separated('=')
			.push(self.name)
			.push(values_columns.name)
			.push_unseparated(',')
			.push_unseparated(self.status)
			.push(values_columns.status)
			.push_unseparated(',')
			.push_unseparated(self.title)
			.push(values_columns.title);
	}

	fn push_unique<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		const UNIQUE: EmployeeColumns<&str> = EmployeeColumns::unique();
		query
			.separated(' ')
			.push(self.id)
			.push("AS")
			.push(UNIQUE.id)
			.push_unseparated(',')
			.push_unseparated(self.name)
			.push("AS")
			.push(UNIQUE.name)
			.push_unseparated(',')
			.push_unseparated(self.status)
			.push("AS")
			.push(UNIQUE.status)
			.push_unseparated(',')
			.push_unseparated(self.title)
			.push("AS")
			.push(UNIQUE.title);
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
