/// # Summary
///
/// A constant to pass to [`write_sql_where_clause`](WriteSqlWhereClause::write_sql_where_clause) which will put `"WHERE"` in front of the
/// clause.
pub const PREFIX_WHERE: Option<&str> = Some("WHERE");

/// # Summary
///
/// A trait to generate SQL `WHERE` clauses.
///
/// Helpful so that multiple implementations of the [`write_sql_where_clause`] method can be
/// created for a builder.
pub trait WriteSqlWhereClause<M>
{
	/// # Summary
	///
	/// Generate an SQL `WHERE` clause for the `column` specified, and append it to the existing
	/// `query`.
	///
	/// PERF: `prefix` is used to reduce the number of [`write!`] by packing more formatting
	///       arguments into the same [`write!`].
	///
	/// # Returns
	///
	/// `true` if anything was written (i.e. `query !=` [`clinvoice_query::Match::Any`]), `false` otherwise.
	fn write_sql_where_clause(
		prefix: Option<&'static str>,
		column: &'static str,
		match_condition: &M,
		query: &mut String,
	) -> bool;
}
