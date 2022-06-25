pub trait TableToSql
{
	/// # Summary
	///
	/// Get the standard alias that can be used to refer to this table.
	///
	/// # Warnings
	///
	/// * Must be unique among other implementors of [`TableToSql`].
	fn table_alias() -> char;

	/// # Summary
	///
	/// Get the name of this table.
	///
	/// # Warnings
	///
	/// * Must be unique among other implementors of [`TableToSql`].
	fn table_name() -> &'static str;
}
