use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

use super::TimesheetColumns;
use crate::fmt::ColumnsToSql;

impl<T> ColumnsToSql for TimesheetColumns<T>
where
	T: Copy + Display,
{
	fn push_columns<Db>(&self, query: &mut QueryBuilder<Db>)
	where
		Db: Database,
	{
		query
			.separated(',')
			.push(self.employee_id)
			.push(self.id)
			.push(self.job_id)
			.push(self.time_begin)
			.push(self.time_end)
			.push(self.work_notes);
	}

	fn push_set<Db>(&self, query: &mut QueryBuilder<Db>, values_ident: impl Copy + Display)
	where
		Db: Database,
	{
		let values_columns = self.scoped(values_ident);
		query
			.separated('=')
			.push(self.employee_id)
			.push(values_columns.employee_id)
			.push_unseparated(',')
			.push_unseparated(self.job_id)
			.push(values_columns.job_id)
			.push_unseparated(',')
			.push_unseparated(self.time_begin)
			.push(values_columns.time_begin)
			.push_unseparated(',')
			.push_unseparated(self.time_end)
			.push(values_columns.time_end)
			.push_unseparated(',')
			.push_unseparated(self.work_notes)
			.push(values_columns.work_notes);
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
