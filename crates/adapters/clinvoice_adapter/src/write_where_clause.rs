mod write_context;

use core::fmt::Display;

use sqlx::{Database, QueryBuilder};
pub use write_context::WriteContext;

/// Implementors of this trait can generate `WHERE` clauses out of some [match
/// condition](clinvoice_match) `M` for the given `Db`'s SQL syntax.
pub trait WriteWhereClause<Db, M>
where
	Db: Database,
{
	/// Append an SQL `WHERE` clause, with a `match_condition` that references `ident`, to `query`.
	///
	/// The [`WriteContext`] for the state of the `query` after all writes is returned.
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: M,
		query: &mut QueryBuilder<Db>,
	) -> WriteContext;
}
