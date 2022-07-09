mod columns_to_sql;
mod table_to_sql;

use crate::fmt::{As, TableToSql, TypeCast, WithIdentifier};

/// The names of the columns of the `jobs` table.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JobColumns<T>
{
	/// The name of the `client_id` column of the `jobs` table.
	pub client_id: T,

	/// The name of the `date_close` column of the `jobs` table.
	pub date_close: T,

	/// The name of the `date_open` column of the `jobs` table.
	pub date_open: T,

	/// The name of the `id` column of the `jobs` table.
	pub id: T,

	/// The name of the `increment` column of the `jobs` table.
	pub increment: T,

	/// The name of the `invoice_date_issued` column of the `jobs` table.
	pub invoice_date_issued: T,

	/// The name of the `invoice_date_paid` column of the `jobs` table.
	pub invoice_date_paid: T,

	/// The name of the `invoice_hourly_rate` column of the `jobs` table.
	pub invoice_hourly_rate: T,

	/// The name of the `notes` column of the `jobs` table.
	pub notes: T,

	/// The name of the `objectives` column of the `jobs` table.
	pub objectives: T,
}

impl<T> JobColumns<T>
{
	/// Returns a [`JobColumns`] which aliases the names of these [`JobColumns`] with the
	/// `aliased` columns provided.
	///
	/// # See also
	///
	/// * [`As`]
	pub fn r#as<TAlias>(self, aliased: JobColumns<TAlias>) -> JobColumns<As<T, TAlias>>
	{
		JobColumns {
			client_id: As(self.client_id, aliased.client_id),
			date_close: As(self.date_close, aliased.date_close),
			date_open: As(self.date_open, aliased.date_open),
			id: As(self.id, aliased.id),
			increment: As(self.increment, aliased.increment),
			invoice_date_issued: As(self.invoice_date_issued, aliased.invoice_date_issued),
			invoice_date_paid: As(self.invoice_date_paid, aliased.invoice_date_paid),
			invoice_hourly_rate: As(self.invoice_hourly_rate, aliased.invoice_hourly_rate),
			notes: As(self.notes, aliased.notes),
			objectives: As(self.objectives, aliased.objectives),
		}
	}

	/// Add a [scope](JobColumns::scope) using the [default alias](TableToSql::default_alias)
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn default_scope(self) -> JobColumns<WithIdentifier<char, T>>
	{
		self.scope(Self::DEFAULT_ALIAS)
	}

	/// Returns a [`JobColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{alias}.{column}`.
	///
	/// # See also
	///
	/// * [`WithIdentifier`]
	pub fn scope<TAlias>(self, alias: TAlias) -> JobColumns<WithIdentifier<TAlias, T>>
	where
		TAlias: Copy,
	{
		JobColumns {
			client_id: WithIdentifier(alias, self.client_id),
			date_open: WithIdentifier(alias, self.date_open),
			date_close: WithIdentifier(alias, self.date_close),
			id: WithIdentifier(alias, self.id),
			increment: WithIdentifier(alias, self.increment),
			invoice_date_issued: WithIdentifier(alias, self.invoice_date_issued),
			invoice_date_paid: WithIdentifier(alias, self.invoice_date_paid),
			invoice_hourly_rate: WithIdentifier(alias, self.invoice_hourly_rate),
			notes: WithIdentifier(alias, self.notes),
			objectives: WithIdentifier(alias, self.objectives),
		}
	}

	/// Returns a [`JobColumns`] which modifies its fields' [`Display`]
	/// implementation to output `{column}::{cast}`.
	///
	/// # See also
	///
	/// * [`TypeCast`]
	pub fn typecast<TCast>(self, cast: TCast) -> JobColumns<TypeCast<T, TCast>>
	where
		TCast: Copy,
	{
		JobColumns {
			client_id: TypeCast(self.client_id, cast),
			date_open: TypeCast(self.date_open, cast),
			date_close: TypeCast(self.date_close, cast),
			id: TypeCast(self.id, cast),
			increment: TypeCast(self.increment, cast),
			invoice_date_issued: TypeCast(self.invoice_date_issued, cast),
			invoice_date_paid: TypeCast(self.invoice_date_paid, cast),
			invoice_hourly_rate: TypeCast(self.invoice_hourly_rate, cast),
			notes: TypeCast(self.notes, cast),
			objectives: TypeCast(self.objectives, cast),
		}
	}
}

impl JobColumns<&'static str>
{
	/// The names of the columns in `jobs` without any aliasing.
	///
	/// # Examples
	///
	/// * See [`JobColumns::unique`].
	pub const fn default() -> Self
	{
		Self {
			client_id: "client_id",
			date_close: "date_close",
			date_open: "date_open",
			id: "id",
			increment: "increment",
			invoice_date_issued: "invoice_date_issued",
			invoice_date_paid: "invoice_date_paid",
			invoice_hourly_rate: "invoice_hourly_rate",
			notes: "notes",
			objectives: "objectives",
		}
	}

	/// Aliases for the columns in `jobs` which are guaranteed to be unique among other [`columns`](super)' `unique` aliases.
	///
	/// # Examples
	///
	/// ```rust
	/// use clinvoice_adapter::{
	///   fmt::{QueryBuilderExt, sql},
	///   schema::columns::{JobColumns, OrganizationColumns},
	/// };
	/// # use pretty_assertions::assert_eq;
	/// use sqlx::{Execute, Postgres, QueryBuilder};
	///
	/// {
	///   let mut query = QueryBuilder::<Postgres>::new(sql::SELECT);
	///
	///   // `sqlx::Row::get` ignores scopes (e.g. "J." in "J.id") so `X.id` and `O.id` clobber each
	///   // other.
	///   assert_eq!(
	///     query
	///       .push_columns(&JobColumns::default().default_scope())
	///       .push_more_columns(&OrganizationColumns::default().default_scope())
	///       .prepare()
	///       .sql(),
	///     " SELECT \
	///         J.client_id,\
	///         J.date_open,\
	///         J.date_close,\
	///         J.id,\
	///         J.increment,\
	///         J.invoice_date_issued,\
	///         J.invoice_date_paid,\
	///         J.invoice_hourly_rate,\
	///         J.notes,\
	///         J.objectives,\
	///         O.id,O.location_id,O.name;"
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
	///       .push_more_columns(&JobColumns::default().default_scope().r#as(JobColumns::unique()))
	///       .prepare()
	///       .sql(),
	///     " SELECT O.id,O.location_id,O.name,\
	///         J.client_id AS unique_4_job_client_id,\
	///         J.date_open AS unique_4_job_date_open,\
	///         J.date_close AS unique_4_job_date_close,\
	///         J.id AS unique_4_job_id,\
	///         J.increment AS unique_4_job_increment,\
	///         J.invoice_date_issued AS unique_4_job_invoice_date_issued,\
	///         J.invoice_date_paid AS unique_4_job_invoice_date_paid,\
	///         J.invoice_hourly_rate AS unique_4_job_invoice_hourly_rate,\
	///         J.notes AS unique_4_job_notes,\
	///         J.objectives AS unique_4_job_objectives;"
	///   );
	/// }
	/// ```
	pub const fn unique() -> Self
	{
		Self {
			client_id: "unique_4_job_client_id",
			date_close: "unique_4_job_date_close",
			date_open: "unique_4_job_date_open",
			id: "unique_4_job_id",
			increment: "unique_4_job_increment",
			invoice_date_issued: "unique_4_job_invoice_date_issued",
			invoice_date_paid: "unique_4_job_invoice_date_paid",
			invoice_hourly_rate: "unique_4_job_invoice_hourly_rate",
			notes: "unique_4_job_notes",
			objectives: "unique_4_job_objectives",
		}
	}
}
