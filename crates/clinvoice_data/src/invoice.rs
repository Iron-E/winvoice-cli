#[cfg(feature="serde_support")]
mod de_ser;
mod hash;

use std::borrow::Cow;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rusty_money::Money;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

/// # Summary
///
/// An `Invoice` represents the accounts receivable for the user or their employer.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Invoice
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
	pub date_issued: Option<DateTime<Utc>>,

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
	pub date_paid: Option<DateTime<Utc>>,

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
struct MockMoney<'currency>
{
	amount: Decimal,
	currency: Cow<'currency, str>,
}

impl From<&Money> for MockMoney<'_>
{
	fn from(money: &Money) -> Self
	{
		return Self
		{
			amount: *money.amount(),
			currency: Cow::Borrowed(money.currency().name),
		};
	}
}
