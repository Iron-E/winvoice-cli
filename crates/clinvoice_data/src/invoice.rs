use crate::{InvoiceDate, Money};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An `Invoice` represents the accounts receivable for the user or their employer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Invoice
{
	/// # Summary
	///
	/// The date upon which the [`Invoice`] was sent to and paid by the client.
	pub date: Option<InvoiceDate>,

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
