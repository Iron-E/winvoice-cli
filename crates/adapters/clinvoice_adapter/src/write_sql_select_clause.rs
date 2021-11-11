/// # Summary
///
/// A trait to generate SQL `SELECT` clauses.
///
/// Helpful so that multiple implementations of the [`write_sql_select_clause`] method can be
/// created for a builder.
pub trait WriteSqlSelectClause
{
	/// # Summary
	///
	/// Return an SQL `SELECT` clause for the `columns` specified.
	fn write_sql_select_clause<const N: usize>(columns: [&'static str; N]) -> String;
}
