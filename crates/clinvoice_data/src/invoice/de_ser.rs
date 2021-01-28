use super::{Invoice, MockMoney};
use std::borrow::Cow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use rusty_money::{money, Money};

#[derive(Deserialize, Serialize)]
struct MockInvoice<'currency>
{
	date_issued: Option<DateTime<Utc>>,
	date_paid: Option<DateTime<Utc>>,
	hourly_rate: MockMoney<'currency>,
}

impl<'de> Deserialize<'de> for Invoice
{
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where
		D : Deserializer<'de>,
	{
		let mock = MockInvoice::deserialize(deserializer)?;

		return Ok(Self
		{
			date_issued: mock.date_issued,
			date_paid: mock.date_paid,
			hourly_rate: money!(mock.hourly_rate.amount, mock.hourly_rate.currency),
		});
	}
}

impl Serialize for Invoice
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
		S : Serializer,
	{
		return MockInvoice
		{
			date_issued: self.date_issued,
			date_paid: self.date_paid,
			hourly_rate: MockMoney
			{
				amount: *self.hourly_rate.amount(),
				currency: Cow::Borrowed(self.hourly_rate.currency().name),
			},
		}.serialize(serializer);
	}
}
