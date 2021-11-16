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
	/// Generate an SQL `WHERE` clause for the `column` specified, and append it to the existing
	/// `query`.
	///
	/// Will skip writing the keyword `WHERE` if `keyword_written`.
	fn write_where_clause(
		context: WriteContext,
		column: &str,
		match_condition: M,
		query: &mut String,
	) -> bool;
}
