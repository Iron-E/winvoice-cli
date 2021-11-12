use std::fmt::{Error, Result, Write};

/// # Summary
///
/// A trait to generate SQL `JOIN` clauses.
///
/// Helpful so that multiple implementations of the [`write_sql_join_clause`] method can be
/// specified.
pub trait WriteSqlJoinClause
{
	/// # Summary
	///
	/// Generate an SQL `JOIN` clause to join two tables, and [`write!`] it to the existing `sql`.
	///
	/// The `join_table` must be given a `join_alias` so that it can be referenced on the
	/// `join_column`. The `base_column` is assumed to have its alias included.
	///
	/// # Errors
	///
	/// Returns [an error](Error) if `join_alias` [is empty](core::str::is_empty).
	///
	/// # Example
	///
	/// In the following snippet:
	///
	/// * `join_table` is `"bar"`
	/// * `join_table` is `'B'`
	/// * `join_column` is `"foo_id"`
	/// * `base_column` is `"F.id"`
	///
	/// ```ignore
	/// JOIN bar B ON (B.foo_id = F.id)
	/// ```
	fn write_sql_join_clause(
		sql: &mut String,
		join_table: &'static str,
		join_alias: &'static str,
		join_column: &'static str,
		base_column: &'static str,
	) -> Result
	{
		if join_alias.is_empty()
		{
			return Err(Error);
		}

		write!(
			sql,
			" JOIN {} {alias} ON ({alias}.{} = {})",
			join_table,
			join_column,
			base_column,
			alias = join_alias
		)
	}
}

#[cfg(test)]
mod tests
{
	use super::{Error, WriteSqlJoinClause};

	#[test]
	fn write_sql_join_clause()
	{
		struct Foo;
		impl WriteSqlJoinClause for Foo {}

		let mut test = String::new();
		Foo::write_sql_join_clause(&mut test, "bar", "B", "foo_id", "F.id").unwrap();
		assert_eq!(test, String::from(" JOIN bar B ON (B.foo_id = F.id)"));
		assert_eq!(
			Foo::write_sql_join_clause(&mut test, "bar", "", "foo_id", "F.id"),
			Err(Error)
		);
	}
}
