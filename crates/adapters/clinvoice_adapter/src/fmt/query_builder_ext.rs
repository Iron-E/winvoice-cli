use core::fmt::Display;

use sqlx::{database::HasArguments, query::Query, Database, QueryBuilder};

use super::{sql, ColumnsToSql, TableToSql};

pub trait QueryBuilderExt<'args>
{
	type Db: Database;

	/// # Summary
	///
	/// Add a semicolon to the end of the current query and then [build](QueryBuilder::build) it.
	fn prepare(&mut self) -> Query<Self::Db, <Self::Db as HasArguments<'args>>::Arguments>;

	/// # Summary
	///
	/// Use [`ColumnsToSql::push_to`] on this query.
	fn push_columns<T>(&mut self, columns: &T) -> &mut Self
	where
		T: ColumnsToSql;

	/// # Summary
	///
	/// Push `" JOIN {TTable::TABLE_NAME} {TTable::table_alias()} ON ({left} = {right})"`.
	fn push_default_equijoin<TLeft, TRight, TTable>(
		&mut self,
		left: TLeft,
		right: TRight,
	) -> &mut Self
	where
		TLeft: Display,
		TRight: Display,
		TTable: TableToSql,
	{
		self.push_equijoin(TTable::TABLE_NAME, TTable::DEFAULT_ALIAS, left, right)
	}

	/// # Summary
	///
	/// Push `" FROM {T::TABLE_NAME} {T::table_alias()}"`.
	fn push_default_from<T>(&mut self) -> &mut Self
	where
		T: TableToSql,
	{
		self.push_from(T::TABLE_NAME, T::DEFAULT_ALIAS)
	}

	/// # Summary
	///
	/// Push `"left = right"`.
	fn push_equal<TLeft, TRight>(&mut self, left: TLeft, right: TRight) -> &mut Self
	where
		TLeft: Display,
		TRight: Display;

	/// # Summary
	///
	/// Push `" JOIN {table_ident} {table_alias} ON ({left} = {right})"`.
	fn push_equijoin<TAlias, TIdent, TLeft, TRight>(
		&mut self,
		table_ident: TIdent,
		table_alias: TAlias,
		left: TLeft,
		right: TRight,
	) -> &mut Self
	where
		TAlias: Display,
		TIdent: Display,
		TLeft: Display,
		TRight: Display;

	/// # Summary
	///
	/// Push `" FROM {table_ident} {table_alias}"`.
	fn push_from<TAlias, TIdent>(&mut self, table_ident: TIdent, table_alias: TAlias) -> &mut Self
	where
		TAlias: Display,
		TIdent: Display;

	/// # Summary
	///
	/// Push a comma and then [`push_columns`](QueryBuilderExt::push_columns).
	fn push_more_columns<T>(&mut self, columns: &T) -> &mut Self
	where
		T: ColumnsToSql;
}

impl<'args, Db> QueryBuilderExt<'args> for QueryBuilder<'args, Db>
where
	Db: Database,
{
	type Db = Db;

	fn prepare(&mut self) -> Query<Db, <Db as HasArguments<'args>>::Arguments>
	{
		self.push(';').build()
	}

	fn push_columns<T>(&mut self, columns: &T) -> &mut Self
	where
		T: ColumnsToSql,
	{
		columns.push_to(self);
		self
	}

	fn push_equal<TLeft, TRight>(&mut self, left: TLeft, right: TRight) -> &mut Self
	where
		TLeft: Display,
		TRight: Display,
	{
		self.separated('=').push(left).push(right);
		self
	}

	fn push_equijoin<TAlias, TIdent, TLeft, TRight>(
		&mut self,
		table_ident: TIdent,
		table_alias: TAlias,
		left: TLeft,
		right: TRight,
	) -> &mut Self
	where
		TAlias: Display,
		TIdent: Display,
		TLeft: Display,
		TRight: Display,
	{
		self
			.push(sql::JOIN)
			.separated(' ')
			.push(table_ident)
			.push(table_alias)
			.push("ON (");

		self.push_equal(left, right).push(')')
	}

	fn push_from<TAlias, TIdent>(&mut self, table_ident: TIdent, table_alias: TAlias) -> &mut Self
	where
		TAlias: Display,
		TIdent: Display,
	{
		self
			.push(sql::FROM)
			.separated(' ')
			.push(table_ident)
			.push(table_alias);

		self
	}

	fn push_more_columns<T>(&mut self, columns: &T) -> &mut Self
	where
		T: ColumnsToSql,
	{
		self.push(',').push_columns(columns)
	}
}
