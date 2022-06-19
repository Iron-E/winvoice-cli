use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::ContactColumns;
use crate::schema::columns::ColumnsToSql;

impl<T> ColumnsToSql for ContactColumns<T>
where
	T: Copy + Display,
{
	fn push_columns<Db>(&self, query: &mut QueryBuilder<Db>)
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

	fn push_set<Db>(&self, query: &mut QueryBuilder<Db>, values_ident: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scoped(values_ident);
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
			.push(self.scoped(table_ident).label)
			.push(self.scoped(values_ident).label);
	}
}
