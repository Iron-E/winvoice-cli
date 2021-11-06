pub trait WriteSql<Q>
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
	fn write_where(
		column: &'static str,
		prefix: Option<&'static str>,
		query: &Q,
		sql: &mut String,
	) -> bool;
}
