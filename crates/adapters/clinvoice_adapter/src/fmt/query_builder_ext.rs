use core::fmt::Display;

use sqlx::{database::HasArguments, query::Query, Database, QueryBuilder};

pub trait QueryBuilderExt<'args>
{
	type Db: Database;

	/// # Summary
	///
	/// Add a semicolon to the end of the current query and then [build](QueryBuilder::build) it.
	fn prepare(&mut self) -> Query<Self::Db, <Self::Db as HasArguments<'args>>::Arguments>;

	/// # Summary
	///
	/// Push `" FROM {table_ident} {table_alias}"`.
	fn push_from<TAlias, TIdent>(&mut self, table_ident: TIdent, table_alias: TAlias) -> &mut Self
	where
		TAlias: Display,
		TIdent: Display;

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
	/// Push `" LEFT JOIN {table_ident} {table_alias} ON ({left} = {right})"`.
	fn push_left_equijoin<TAlias, TIdent, TLeft, TRight>(
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

	fn push_equal<TLeft, TRight>(&mut self, left: TLeft, right: TRight) -> &mut Self
	where
		TLeft: Display,
		TRight: Display,
	{
		self.separated('=').push(left).push(right);
		self
	}

	fn push_from<TAlias, TIdent>(&mut self, table_ident: TIdent, table_alias: TAlias) -> &mut Self
	where
		TAlias: Display,
		TIdent: Display,
	{
		self
			.separated(' ')
			.push(" FROM")
			.push(table_ident)
			.push(table_alias);

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
			.separated(' ')
			.push(" JOIN")
			.push(table_ident)
			.push(table_alias)
			.push("ON (");

		self.push_equal(left, right).push(')')
	}

	fn push_left_equijoin<TAlias, TIdent, TLeft, TRight>(
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
			.push(" LEFT")
			.push_equijoin(table_ident, table_alias, left, right)
	}
}
