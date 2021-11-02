pub trait WriteSql<Q>
{
	/// # Summary
	///
	/// Generate an `sql` `WHERE` clause for the `column` specified.
	///
	/// PERF: `prefix` is used to reduce the number of `write!`s, by combining combining multiple
	///       formatting arguments into the same `String::push` rather than `push`ing more
	///       frequently.
	fn write_where(column: &'static str, prefix: Option<&'static str>, query: &Q, sql: &mut String);
}
