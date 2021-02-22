mod display;

use chrono::{DateTime, Utc};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An `InvoiceDate` represents the dates which an invoice was sent to or paid by a client.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct InvoiceDate
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
	pub issued: DateTime<Utc>,

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
	pub paid: Option<DateTime<Utc>>,
}
