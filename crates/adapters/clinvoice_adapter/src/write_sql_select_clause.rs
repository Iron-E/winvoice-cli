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
	fn write_sql_select_clause<const LEN: usize>(columns: [&'static str; LEN]) -> String
	{
		if !columns.is_empty()
		{
			let mut output = columns.join(",");
			output.insert_str(0, "SELECT ");
			output
		}
		else
		{
			Self::write_sql_select_clause(["*"])
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::WriteSqlSelectClause;

	#[test]
	fn write_sql_select_clause()
	{
		struct Foo;
		impl WriteSqlSelectClause for Foo {}

		assert_eq!(
			Foo::write_sql_select_clause(["id", "foo"]),
			String::from("SELECT id,foo"),
		);

		assert_eq!(
			Foo::write_sql_select_clause(["*"]),
			Foo::write_sql_select_clause([]),
		);
	}
}
