/// # Summary
///
/// A constant to pass to [`write_where_clause`](WriteWhereClause::write_where_clause) which will put `"WHERE"` in front of the
/// clause.
pub const PREFIX_WHERE: &str = "WHERE";

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
	///
	/// # Returns
	///
	/// `true` if anything was written, `false` otherwise.
	fn write_where_clause(
		keyword_written: bool,
		column: &'static str,
		match_condition: &M,
		query: &mut String,
	) -> bool;
}
