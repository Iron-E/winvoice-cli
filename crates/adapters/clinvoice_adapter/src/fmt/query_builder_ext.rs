mod sealed
{
	use sqlx::{Database, QueryBuilder};

	pub trait Sealed {}
	impl<'args, Db> Sealed for QueryBuilder<'args, Db> where Db: Database {}
}

use core::fmt::Display;

use sqlx::{database::HasArguments, query::Query, Database, QueryBuilder};

use super::{sql, ColumnsToSql, TableToSql};

/// An extension to [`QueryBuilder`] that expands upon
/// [`insert_values`](QueryBuilder::insert_values`) by enabling it to generate other SQL clauses as
/// well.
pub trait QueryBuilderExt<'args>: sealed::Sealed
{
	type Db: Database;

	/// Add a semicolon to the end of the current query and then [build](QueryBuilder::build) it.
	///
	/// # Examples
	///
	/// * See [`EmployeeColumns::unique`](crate::schema::columns::EmployeeColumns::unique).
	fn prepare(&mut self) -> Query<Self::Db, <Self::Db as HasArguments<'args>>::Arguments>;

	/// [`ColumnsToSql::push_to`] this query.
	///
	/// # Examples
	///
	/// * See [`EmployeeColumns::unique`](crate::schema::columns::EmployeeColumns::unique).
	fn push_columns<T>(&mut self, columns: &T) -> &mut Self
	where
		T: ColumnsToSql;

	/// Push `" JOIN {TTable::TABLE_NAME} {TTable::TABLE_ALIAS} ON ({left} = {right})"`.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::{
	///   fmt::{QueryBuilderExt, sql, WithIdentifier},
	///   schema::columns::{LocationColumns, OrganizationColumns},
	/// };
	/// # use pretty_assertions::assert_eq;
	/// use sqlx::{Execute, Postgres, QueryBuilder};
	///
	/// let organization_columns = OrganizationColumns::default().default_scope();
	/// let location_columns = LocationColumns::default().default_scope();
	/// let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	/// assert_eq!(
	///   query
	///     .push_columns(&organization_columns)
	///     .push_default_from::<OrganizationColumns<&str>>()
	///     .push_default_equijoin::<LocationColumns<&str>, _, _>(
	///       location_columns.id,
	///       organization_columns.location_id,
	///     )
	///     .prepare()
	///     .sql(),
	///   " SELECT O.id,O.location_id,O.name \
	///     FROM organizations O \
	///     JOIN locations L ON (L.id=O.location_id);"
	/// );
	/// ```
	fn push_default_equijoin<TTable, TLeft, TRight>(
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

	/// Push `" FROM {T::TABLE_NAME} {T::TABLE_ALIAS}"`.
	///
	/// # Examples
	///
	/// * See [`QueryBuilderExt::push_default_equijoin`]
	fn push_default_from<T>(&mut self) -> &mut Self
	where
		T: TableToSql,
	{
		self.push_from(T::TABLE_NAME, T::DEFAULT_ALIAS)
	}

	/// Push `"left = right"`.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::{
	///   fmt::{QueryBuilderExt, sql, WithIdentifier},
	///   schema::columns::OrganizationColumns,
	/// };
	/// # use pretty_assertions::assert_eq;
	/// use sqlx::{Execute, Postgres, QueryBuilder};
	///
	/// let organization_columns = OrganizationColumns::default().default_scope();
	/// let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	/// assert_eq!(
	///   query
	///     .push_columns(&organization_columns)
	///     .push_default_from::<OrganizationColumns<&str>>()
	///     .push(sql::WHERE)
	///     .push_equal(organization_columns.id, 3)
	///     .prepare()
	///     .sql(),
	///   " SELECT O.id,O.location_id,O.name FROM organizations O WHERE O.id=3;"
	/// );
	/// ```
	fn push_equal<TLeft, TRight>(&mut self, left: TLeft, right: TRight) -> &mut Self
	where
		TLeft: Display,
		TRight: Display;

	/// Push `" JOIN {table_ident} {table_alias} ON ({left} = {right})"`.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::fmt::{QueryBuilderExt, sql, WithIdentifier};
	/// # use pretty_assertions::assert_eq;
	/// use sqlx::{Execute, Postgres, QueryBuilder};
	///
	/// let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	/// assert_eq!(
	///   query
	///     .push_from("foo", 'F')
	///     .push_equijoin("bar", 'B', "B.foo_id", "F.id")
	///     .prepare()
	///     .sql(),
	///   " SELECT  FROM foo F JOIN bar B ON (B.foo_id=F.id);"
	/// );
	/// ```
	fn push_equijoin<TIdent, TAlias, TLeft, TRight>(
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

	/// Push `" FROM {table_ident} {table_alias}"`.
	///
	/// # Examples
	///
	/// * See [`QueryBuilderExt::push_equijoin`]
	fn push_from<TIdent, TAlias>(&mut self, table_ident: TIdent, table_alias: TAlias) -> &mut Self
	where
		TAlias: Display,
		TIdent: Display;

	/// Push a comma and then [`push_columns`](QueryBuilderExt::push_columns).
	///
	/// # Examples
	///
	/// * See [`QueryBuilderExt::push_columns`]
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

	fn push_equijoin<TIdent, TAlias, TLeft, TRight>(
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

	fn push_from<TIdent, TAlias>(&mut self, table_ident: TIdent, table_alias: TAlias) -> &mut Self
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
