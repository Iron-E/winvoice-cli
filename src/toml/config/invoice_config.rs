use rusty_money::Iso;

/// # Summary
///
/// The `InvoiceConfig` contains settings related to payment.
pub struct InvoiceConfig
{
	/// # Summary
	///
	/// The primary currency which is used by clients.
	///
	/// # Remarks
	///
	/// Upon `clinvoice export`ing the invoice, it will be possible to pass a flag to serialization
	/// that changes the currency for one export only:
	///
	/// ```sh
	/// clinvoice export 005 --currency CAD
	/// ```
	pub currency: Iso,
}
