mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{As, TableToSql, WithIdentifier};

/// The names of the columns of the `employees` table.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EmployeeColumns<T>
{
	/// The name of the `id` column of the `employees` table.
	pub id: T,

	/// The name of the `name` column of the `employees` table.
	pub name: T,

	/// The name of the `status` column of the `employees` table.
	pub status: T,

	/// The name of the `title` column of the `employees` table.
	pub title: T,
}

impl<T> EmployeeColumns<T>
{
	/// Returns a [`EmployeeColumns`] which aliases the names of these [`EmployeeColumns`] with the
	/// `aliased` columns provided.
	///
	/// # See also
	///
	/// * [`As`]
	pub fn r#as<TAlias>(self, aliased: EmployeeColumns<TAlias>) -> EmployeeColumns<As<T, TAlias>>
	{
		EmployeeColumns {
			id: As(self.id, aliased.id),
			name: As(self.name, aliased.name),
			status: As(self.status, aliased.status),
			title: As(self.title, aliased.title),
		}
	}

	/// Add a [scope](EmployeeColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn default_scope(self) -> EmployeeColumns<WithIdentifier<char, T>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// Returns a [`EmployeeColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn scope<TAlias>(self, alias: TAlias) -> EmployeeColumns<WithIdentifier<TAlias, T>>
	where
		TAlias: Copy,
	{
		EmployeeColumns {
			id: WithIdentifier(alias, self.id),
			name: WithIdentifier(alias, self.name),
			status: WithIdentifier(alias, self.status),
			title: WithIdentifier(alias, self.title),
		}
	}
}

impl EmployeeColumns<&'static str>
{
	/// The names of the columns in `employees` without any aliasing.
	///
	/// # Examples
	///
	/// * See [`EmployeeColumns::unique`].
	pub const fn default() -> Self
	{
		Self {
			id: "id",
			name: "name",
			status: "status",
			title: "title",
		}
	}

	/// Aliases for the columns in `employees` which are guaranteed to be unique among other [`columns`](super)'s `unique` aliases.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::{
	///   fmt::{QueryBuilderExt, sql},
	///   schema::columns::{EmployeeColumns, OrganizationColumns},
	/// };
	/// # use pretty_assertions::assert_eq;
	/// use sqlx::{Execute, Postgres, QueryBuilder};
	///
	/// {
	///   let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	///   // `sqlx::Row::get` ignores scopes (e.g. "E." in "E.id") so "E.id" and "O.id", as well as
	///   // "E.name" and "O.name", clobber each other.
	///   assert_eq!(
	///     query
	///       .push_columns(&EmployeeColumns::default().default_scope())
	///       .push_more_columns(&OrganizationColumns::default().default_scope())
	///       .prepare()
	///       .sql(),
	///     " SELECT E.id,E.name,E.status,E.title,O.id,O.location_id,O.name;"
	///   );
	/// }
	///
	/// {
	///   let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	///   // no clobbering
	///   assert_eq!(
	///     query
	///       .push_columns(&OrganizationColumns::default().default_scope())
	///       .push_more_columns(&EmployeeColumns::default().default_scope().r#as(EmployeeColumns::unique()))
	///       .prepare()
	///       .sql(),
	///     " SELECT O.id,O.location_id,O.name,\
	///         E.id AS unique_2_employee_id,\
	///         E.name AS unique_2_employee_name,\
	///         E.status AS unique_2_employee_status,\
	///         E.title AS unique_2_employee_title;"
	///   );
	/// }
	/// ```
	pub const fn unique() -> Self
	{
		Self {
			id: "unique_2_employee_id",
			name: "unique_2_employee_name",
			status: "unique_2_employee_status",
			title: "unique_2_employee_title",
		}
	}
}
