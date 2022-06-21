use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::JobColumns;
use crate::fmt::{ColumnsToSql, QueryBuilderExt};

impl<T> ColumnsToSql for JobColumns<T>
where
	T: Copy + Display,
{
	fn push_to<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push(self.client_id)
			.push(self.date_open)
			.push(self.date_close)
			.push(self.id)
			.push(self.increment)
			.push(self.invoice_date_issued)
			.push(self.invoice_date_paid)
			.push(self.invoice_hourly_rate)
			.push(self.notes)
			.push(self.objectives);
	}

	fn push_set_to<Db>(&self, query: &mut QueryBuilder<Db>, values_alias: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scope(values_alias);
		query
			.push_equal(self.client_id, values_columns.client_id)
			.push(',')
			.push_equal(self.date_open, values_columns.date_open)
			.push(',')
			.push_equal(self.date_close, values_columns.date_close)
			.push(',')
			.push_equal(self.increment, values_columns.increment)
			.push(',')
			.push_equal(self.invoice_date_issued, values_columns.invoice_date_issued)
			.push(',')
			.push_equal(self.invoice_date_paid, values_columns.invoice_date_paid)
			.push(',')
			.push_equal(self.invoice_hourly_rate, values_columns.invoice_hourly_rate)
			.push(',')
			.push_equal(self.notes, values_columns.notes)
			.push(',')
			.push_equal(self.objectives, values_columns.objectives);
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
