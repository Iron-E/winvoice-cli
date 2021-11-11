/// # Summary
///
/// A trait to generate SQL `from` clauses.
///
/// Helpful so that multiple implementations of the [`write_sql_from_clause`] method can be
/// created for a builder.
pub trait WriteSqlFromClause
{
	/// # Summary
	///
	/// Generate an SQL `FROM` clause to pull data from a `table`, and [`write!`] it to the existing `sql`.
	///
	/// You may `alias` a `table` in order to use it in a `JOIN` clause:
	///
	/// # Example
	///
	/// In the following example:
	///
	/// * `table` is `"foo"`
	/// * `alias` is `Some('F')`
	///
	/// ```ignore
	/// FROM foo F
	/// ```
	fn write_sql_from_clause(sql: &mut String, table: &'static str, alias: Option<char>);
}
