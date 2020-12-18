/// # Summary
///
/// The `CLInvoiceConfig` contains general settings that affect all areas of the application. Any
/// setting with particular scope may be found in _its_ dedicated structure.
pub struct CLInvoiceConfig<'directory>
{
	/// # Summary
	///
	/// The directory used for storing serialized [TOML](super::toml) structures.
	///
	/// # Remarks
	///
	/// While some users may wish to store each invoice in a particular directory which is related
	/// to the work being performed, it is desirable that the application may be able to
	/// consistently access every created invoice.
	///
	/// If a user wishes to `clinvoice export` to another directory, that is entirely feasible.
	/// Exports are not tracked by `clinvoice`, and as such, have no default directory.
	///
	/// # Example
	///
	/// > __Note:__ if the path specified does not exist, it will be created!
	///
	/// ```sh
	/// clinvoice config -d ~/Documents/CLInvoice
	/// ```
	pub directory: &'directory str
}
