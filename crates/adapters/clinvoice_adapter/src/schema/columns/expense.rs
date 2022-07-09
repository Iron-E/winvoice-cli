mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{As, TableToSql, TypeCast, WithIdentifier};

/// The names of the columns of the `expenses` table.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExpenseColumns<T>
{
	/// The name of the `category` column of the `expenses` table.
	pub category: T,

	/// The name of the `cost` column of the `expenses` table.
	pub cost: T,

	/// The name of the `description` column of the `expenses` table.
	pub description: T,

	/// The name of the `id` column of the `expenses` table.
	pub id: T,

	/// The name of the `timesheet_id` column of the `expenses` table.
	pub timesheet_id: T,
}

impl<T> ExpenseColumns<T>
{
	/// Returns a [`ExpenseColumns`] which aliases the names of these [`ExpenseColumns`] with the
	/// `aliased` columns provided.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::schema::columns::ExpenseColumns;
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   ExpenseColumns::default()
	///     .default_scope()
	///     .r#as(ExpenseColumns {
	///       category: "one",
	///       cost: "two",
	///       description: "three",
	///       id: "four",
	///       timesheet_id: "five",
	///     })
	///     .id
	///     .to_string(),
	///   "X.id AS four",
	/// );
	/// ```
	pub fn r#as<TAlias>(self, aliased: ExpenseColumns<TAlias>) -> ExpenseColumns<As<T, TAlias>>
	{
		ExpenseColumns {
			category: As(self.category, aliased.category),
			cost: As(self.cost, aliased.cost),
			description: As(self.description, aliased.description),
			id: As(self.id, aliased.id),
			timesheet_id: As(self.timesheet_id, aliased.timesheet_id),
		}
	}

	/// Add a [scope](ExpenseColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # Examples
	///
	/// * See [`ExpenseColumns::r#as`].
	pub fn default_scope(self) -> ExpenseColumns<WithIdentifier<char, T>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// Returns a [`ExpenseColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # Examples
	///
	/// * See [`ExpenseColumns::default_scope`].
	pub fn scope<TAlias>(self, alias: TAlias) -> ExpenseColumns<WithIdentifier<TAlias, T>>
	where
		TAlias: Copy,
	{
		ExpenseColumns {
			id: WithIdentifier(alias, self.id),
			timesheet_id: WithIdentifier(alias, self.timesheet_id),
			category: WithIdentifier(alias, self.category),
			cost: WithIdentifier(alias, self.cost),
			description: WithIdentifier(alias, self.description),
		}
	}

	/// Returns a [`ExpenseColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::schema::columns::ExpenseColumns;
	/// # use pretty_assertions::assert_eq;
	///
	/// assert_eq!(
	///   ExpenseColumns::default().typecast("numeric").cost.to_string(),
	///   " CAST (cost AS numeric)",
	/// );
	/// ```
	pub fn typecast<TCast>(self, cast: TCast) -> ExpenseColumns<TypeCast<T, TCast>>
	where
		TCast: Copy,
	{
		ExpenseColumns {
			id: TypeCast(self.id, cast),
			timesheet_id: TypeCast(self.timesheet_id, cast),
			category: TypeCast(self.category, cast),
			cost: TypeCast(self.cost, cast),
			description: TypeCast(self.description, cast),
		}
	}
}

impl ExpenseColumns<&'static str>
{
	/// The names of the columns in `expenses` without any aliasing.
	///
	/// # Examples
	///
	/// * See [`ExpenseColumns::r#as`].
	pub const fn default() -> Self
	{
		Self {
			category: "category",
			cost: "cost",
			description: "description",
			id: "id",
			timesheet_id: "timesheet_id",
		}
	}

	/// Aliases for the columns in `expenses` which are guaranteed to be unique among other [`columns`](super)'s `unique` aliases.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::{
	///   fmt::{QueryBuilderExt, sql},
	///   schema::columns::{ExpenseColumns, OrganizationColumns},
	/// };
	/// # use pretty_assertions::assert_eq;
	/// use sqlx::{Execute, Postgres, QueryBuilder};
	///
	/// {
	///   let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	///   // `sqlx::Row::get` ignores scopes (e.g. "X." in "X.id") so "X.id" and "O.id" clobber each
	///   // other.
	///   assert_eq!(
	///     query
	///       .push_columns(&ExpenseColumns::default().default_scope())
	///       .push_more_columns(&OrganizationColumns::default().default_scope())
	///       .prepare()
	///       .sql(),
	///     " SELECT X.category,X.cost,X.description,X.id,X.timesheet_id,O.id,O.location_id,O.name;"
	///   );
	/// }
	///
	/// {
	///   let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	///   // no clobbering
	///   assert_eq!(
	///     query
	///       .push_columns(&ExpenseColumns::default().default_scope().r#as(ExpenseColumns::unique()))
	///       .push_more_columns(&OrganizationColumns::default().default_scope())
	///       .prepare()
	///       .sql(),
	///     " SELECT \
	///         X.category AS unique_3_expense_category,\
	///         X.cost AS unique_3_expense_cost,\
	///         X.description AS unique_3_expense_description,\
	///         X.id AS unique_3_expense_id,\
	///         X.timesheet_id AS unique_3_expense_timesheet_id,\
	///         O.id,O.location_id,O.name;"
	///   );
	/// }
	/// ```
	pub const fn unique() -> Self
	{
		Self {
			category: "unique_3_expense_category",
			cost: "unique_3_expense_cost",
			description: "unique_3_expense_description",
			id: "unique_3_expense_id",
			timesheet_id: "unique_3_expense_timesheet_id",
		}
	}
}
