mod write_context;

pub use write_context::WriteContext;

/// # Summary
///
/// A trait to generate SQL `WHERE` clauses.
///
/// Helpful so that multiple implementations of the [`write_where_clause`] method can be
/// created for a builder.
pub trait WriteWhereClause<M>
{
	/// # Summary
	///
	/// Generate an SQL `WHERE` clause for the `alias` specified, and append it to the existing
	/// `query` after writing the `context`'s [prefix](WriteContext::get_prefix).
	///
	/// * Depending on implementation, `alias` must either be the `alias` of [`WriteFromClause::write_from_clause`]/[`WriteJoinClause::write_join_clause`] (e.g. "P", ""), or some column of a table that should be queried (e.g. "P.id", "id").
	///
	/// # Return
	///
	/// The [`WriteContext`] that the `query` will be in after this operation. Can be passed into
	/// the `context` argument of another call to this same method.
	fn write_where_clause(
		context: WriteContext,
		alias: &str,
		match_condition: M,
		query: &mut String,
	) -> WriteContext;
}
