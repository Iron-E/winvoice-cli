#[macro_export]
macro_rules! newtype_invoice
{
	($name:ident) =>
	{
		use clinvoice_data::{chrono::TimeZone, Invoice};

		/// # Summary
		///
		/// A wrapper around [`Invoice`] for use with TomlDB.
		pub struct $name<TZone> (Invoice<TZone>) where TZone : TimeZone;

		impl<TZone> From<Invoice<TZone>> for $name<TZone> where TZone : TimeZone
		{
			fn from(invoice: Invoice<TZone>) -> Self
			{
				return $name (invoice);
			}
		}
	}
}
