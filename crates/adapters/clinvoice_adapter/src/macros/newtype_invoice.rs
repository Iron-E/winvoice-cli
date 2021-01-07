#[macro_export]
macro_rules! NewtypeInvoice
{
	($name:ident, $T:ident) =>
	{
		use clinvoice_data::{chrono::TimeZone, Invoice};

		/// # Summary
		///
		/// A wrapper around [`Invoice`] for use with TomlDB.
		pub struct $name<$T> (Invoice<$T>) where $T : TimeZone;

		impl<$T> From<Invoice<$T>> for $name<$T> where $T : TimeZone
		{
			fn from(invoice: Invoice<$T>) -> Self
			{
				return $name (invoice);
			}
		}
	};
}
