mod display;

use chrono::{DateTime, Utc};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

/// Represents the dates which an [`Invoice`][invoice] was sent to or paid by a client.
///
/// This is a separate structure so that it can be wrapped in [`Option`], which uses Rust's type
/// system to ensure the one of the following is always true:
///
/// * There is no `issued` and no `paid` date.
/// * There is an `issued` date, but no `paid` date.
/// * There is an `issued` date and a `paid` date.
///
/// As having a `paid` date without an `issued` date would be invalid state for the
/// [`Invoice`][invoice], the above use of [`Option`] is useful while the [`Invoice`] exists
/// outside of the constraint system of a database.
///
/// # Examples
///
/// ```rust
/// use clinovice_schema::{chrono::Utc, InvoiceDate};
///
/// let _unpaid = InvoiceDate {
///   issued: Utc::now(),
///   paid: None,
/// };
///
/// let _paid = InvoiceDate {
///   issued: Utc::now(),
///   paid: Some(Utc::now()),
/// };
/// ```
///
/// [invoice]: super::Invoice
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InvoiceDate
{
	/// The date that the [`Invoice`](super::Invoice) was sent to the client.
	pub issued: DateTime<Utc>,

	/// The date that the client paid the [`Invoice`](super::Invoice).
	pub paid: Option<DateTime<Utc>>,
}
