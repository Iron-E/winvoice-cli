use core::fmt::Display;

use sqlx::{Database, QueryBuilder};

pub trait QueryBuilderExt
{
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
}

impl<Db> QueryBuilderExt for QueryBuilder<'_, Db>
where
	Db: Database,
{
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
}
