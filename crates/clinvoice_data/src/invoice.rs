use chrono::{DateTime, TimeZone};
use rusty_money::Money;

/// # Summary
///
/// An `Invoice` represents the accounts receivable for the user or their employer.
pub struct Invoice<TZone> where TZone : TimeZone
{
	/// # Summary
	///
	/// The date upon which the [`Invoice`] was sent to the client.
	///
	/// # Remarks
	///
	/// Upon running `clinvoice new`, this field is left blank. This is to signify that the
	/// underlying [`Invoice`] has not been sent to the client.
	///
	/// When running `clinvoice export`, this field will be set automatically to the current date
	/// and time.
	pub date_issued: Option<DateTime<TZone>>,

	/// # Summary
	///
	/// The date upon which the client paid the [`Invoice`].
	///
	/// # Remarks
	///
	/// Upon running `clinvoice new`, this field is left blank. This is to signify that the
	/// underlying [`Invoice`] has not paid by the client.
	///
	/// This field will be updated when running `clinvoice rec`/`receive`
	pub date_paid: Option<DateTime<TZone>>,

	/// # Summary
	///
	/// The amount of money to be charged for one hour of work.
	///
	/// # Configuration
	///
	/// The currency used for this rate can be configured by running:
	///
	/// ```sh
	/// `clinvoice config -c '<char>'`.
	/// ```
	///
	/// ## Example
	///
	/// ```sh
	/// clinvoice config -c '\$'
	/// ```
	pub hourly_rate: Money,
}
