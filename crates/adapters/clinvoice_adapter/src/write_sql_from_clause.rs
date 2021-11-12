use std::fmt::Write;

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
	/// You may `alias` a `table` in order to use it in a `JOIN` clause. Otherwise, let `alias` be
	/// an empty `&str`.
	///
	/// # Example
	///
	/// In the following example:
	///
	/// * `table` is `"foo"`
	/// * `alias` is `"F"`
	///
	/// ```ignore
	/// FROM foo F
	/// ```
	fn write_sql_from_clause(sql: &mut String, table: &'static str, alias: &'static str)
	{
		write!(sql, " FROM {} {}", table, alias).unwrap()
	}
}

#[cfg(test)]
mod tests
{
	use super::WriteSqlFromClause;

	#[test]
	fn write_sql_from_clause()
	{
		struct Foo;
		impl WriteSqlFromClause for Foo {}

		let mut test = String::new();
		Foo::write_sql_from_clause(&mut test, "foo", "F");
		assert_eq!(test, String::from(" FROM foo F"),);

		test.clear();
		Foo::write_sql_from_clause(&mut test, "foo", "");
		assert_eq!(test, String::from(" FROM foo "),);
	}
}
