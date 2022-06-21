use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::ContactColumns;
use crate::fmt::ColumnsToSql;

impl<T> ColumnsToSql for ContactColumns<T>
where
	T: Copy + Display,
{
	fn push<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push(&self.address_id)
			.push(&self.email)
			.push(&self.label)
			.push(&self.other)
			.push(&self.phone);
	}

	fn push_set<Db>(&self, query: &mut QueryBuilder<Db>, values_alias: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scope(values_alias);
		query
			.separated('=')
			.push(self.address_id)
			.push(values_columns.address_id)
			.push_unseparated(',')
			.push_unseparated(self.email)
			.push(values_columns.email)
			.push_unseparated(',')
			.push_unseparated(self.label)
			.push(values_columns.label)
			.push_unseparated(',')
			.push_unseparated(self.other)
			.push(values_columns.other)
			.push_unseparated(',')
			.push_unseparated(self.phone)
			.push(values_columns.phone);
	}

	fn push_unique<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		const UNIQUE: ContactColumns<&str> = ContactColumns::unique();
		query
			.separated(' ')
			.push(self.address_id)
			.push("AS")
			.push(UNIQUE.address_id)
			.push_unseparated(',')
			.push_unseparated(self.email)
			.push("AS")
			.push(UNIQUE.email)
			.push_unseparated(',')
			.push_unseparated(self.label)
			.push("AS")
			.push(UNIQUE.label)
			.push_unseparated(',')
			.push_unseparated(self.other)
			.push("AS")
			.push(UNIQUE.other)
			.push_unseparated(',')
			.push_unseparated(self.phone)
			.push("AS")
			.push(UNIQUE.phone);
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
			.push(self.scope(table_alias).label)
			.push(self.scope(values_alias).label);
	}
}
