use std::borrow::Cow;
use rust_decimal::Decimal;

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Money<'currency>
{
	amount: Decimal,
	currency: Cow<'currency, str>,
}
