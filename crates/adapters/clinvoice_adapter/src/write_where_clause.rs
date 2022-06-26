mod write_context;

use core::fmt::Display;

use sqlx::{Database, QueryBuilder};
pub use write_context::WriteContext;

/// # Summary
///
/// A trait to generate SQL `WHERE` clauses.
///
/// Helpful so that multiple implementations of the [`write_where_clause`] method can be
/// created for a builder.
pub trait WriteWhereClause<Db, M>
where
	Db: Database,
{
	/// # Summary
	///
	/// Generate an SQL `WHERE` clause for the `ident` specified, and append it to the existing
	/// `query` after [displaying](Display) the `context`.
	///
	/// * Depending on implementation, `ident` must either be the alias of a `FROM`/`JOIN` (e.g. 'P', "") or some column of a table that should be queried (e.g. "P.id", "id").
	///
	/// # Return
	///
	/// The [`WriteContext`] that the `query` will be in after this operation. Can be passed into
	/// the `context` argument of another call to this same method.
	fn write_where_clause(
		context: WriteContext,
		ident: impl Copy + Display,
		match_condition: M,
		query: &mut QueryBuilder<Db>,
	) -> WriteContext;
}
