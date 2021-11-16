use std::fmt::{Error, Result, Write};

/// # Summary
///
/// A trait to generate SQL `JOIN` clauses.
///
/// Helpful so that multiple implementations of the [`write_join_clause`] method can be
/// specified.
pub trait WriteJoinClause
{
	/// # Summary
	///
	/// Generate an SQL `JOIN` clause to join two tables, and [`write!`] it to the existing `query`.
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
	/// * `join` is `""`
	/// * `join_table` is `"bar"`
	/// * `join_table` is `'B'`
	/// * `join_column` is `"foo_id"`
	/// * `base_column` is `"F.id"`
	///
	/// ```ignore
	/// JOIN bar B ON (B.foo_id = F.id)
	/// ```
	fn write_join_clause(
		query: &mut String,
		join: &str,
		join_table: &str,
		join_alias: &str,
		join_column: &str,
		base_column: &str,
	) -> Result
	{
		if join_alias.is_empty()
		{
			return Err(Error);
		}

		write!(
			query,
			" {} JOIN {} {alias} ON ({alias}.{} = {})",
			join,
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
	use super::{Error, WriteJoinClause};

	#[test]
	fn write_join_clause()
	{
		struct Foo;
		impl WriteJoinClause for Foo {}

		let mut query = String::new();
		Foo::write_join_clause(&mut query, "", "bar", "B", "foo_id", "F.id").unwrap();
		assert_eq!(query, String::from("  JOIN bar B ON (B.foo_id = F.id)"));

		query.clear();
		Foo::write_join_clause(&mut query, "LEFT", "clumpf", "C", "bar_id", "B.id").unwrap();
		assert_eq!(
			query,
			String::from(" LEFT JOIN clumpf C ON (C.bar_id = B.id)")
		);

		assert_eq!(
			Foo::write_join_clause(&mut query, "", "bar", "", "foo_id", "F.id"),
			Err(Error)
		);
	}
}