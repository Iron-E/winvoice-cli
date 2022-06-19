use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::JobColumns;
use crate::fmt::ColumnsToSql;

impl<T> ColumnsToSql for JobColumns<T>
where
	T: Copy + Display,
{
	fn push_columns<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push_unseparated(self.client_id)
			.push_unseparated(self.date_open)
			.push_unseparated(self.date_close)
			.push_unseparated(self.id)
			.push_unseparated(self.increment)
			.push_unseparated(self.invoice_date_issued)
			.push_unseparated(self.invoice_date_paid)
			.push_unseparated(self.invoice_hourly_rate)
			.push_unseparated(self.notes)
			.push_unseparated(self.objectives);
	}

	fn push_set<Db>(&self, query: &mut QueryBuilder<Db>, values_ident: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scoped(values_ident);
		query
			.separated('=')
			.push(self.client_id)
			.push(values_columns.client_id)
			.push_unseparated(',')
			.push_unseparated(self.date_open)
			.push(values_columns.date_open)
			.push_unseparated(',')
			.push_unseparated(self.date_close)
			.push(values_columns.date_close)
			.push_unseparated(',')
			.push_unseparated(self.increment)
			.push(values_columns.increment)
			.push_unseparated(',')
			.push_unseparated(self.invoice_date_issued)
			.push(values_columns.invoice_date_issued)
			.push_unseparated(',')
			.push_unseparated(self.invoice_date_paid)
			.push(values_columns.invoice_date_paid)
			.push_unseparated(',')
			.push_unseparated(self.invoice_hourly_rate)
			.push(values_columns.invoice_hourly_rate)
			.push_unseparated(',')
			.push_unseparated(self.notes)
			.push(values_columns.notes)
			.push_unseparated(',')
			.push_unseparated(self.objectives)
			.push(values_columns.objectives);
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
