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
	/// Generate an `sql` `WHERE` clause for the `column` specified.
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
		sql: &mut String,
	) -> bool;
}
