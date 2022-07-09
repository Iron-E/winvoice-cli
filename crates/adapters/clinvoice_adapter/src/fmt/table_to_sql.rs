pub(crate) mod sealed
{
	use crate::schema::columns::{
		ContactColumns,
		EmployeeColumns,
		ExpenseColumns,
		JobColumns,
		LocationColumns,
		OrganizationColumns,
		TimesheetColumns,
	};

	pub trait Sealed {}
	impl<T> Sealed for ContactColumns<T> {}
	impl<T> Sealed for EmployeeColumns<T> {}
	impl<T> Sealed for ExpenseColumns<T> {}
	impl<T> Sealed for JobColumns<T> {}
	impl<T> Sealed for LocationColumns<T> {}
	impl<T> Sealed for OrganizationColumns<T> {}
	impl<T> Sealed for TimesheetColumns<T> {}
}

/// Defines attributes of a table in a database which was
/// [`init`](crate::Initializable::init)ialized for use with CLInvoice.
///
/// # Examples
///
/// * See [`QueryBuilderExt::push_default_from`](super::QueryBuilderExt::push_default_from).
pub trait TableToSql: sealed::Sealed
{
	/// Get the standard alias that can be used to refer to this table.
	///
	/// # Warnings
	///
	/// * Must be unique among other implementors of [`TableToSql`].
	const DEFAULT_ALIAS: char;

	/// Get the name of this table.
	///
	/// # Warnings
	///
	/// * Must be unique among other implementors of [`TableToSql`].
	const TABLE_NAME: &'static str;
}
