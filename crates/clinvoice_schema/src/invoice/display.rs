use core::fmt::{Display, Formatter, Result};

use super::{Invoice, InvoiceDate};

impl Display for Invoice
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "Hourly Rate: {}", self.hourly_rate)?;

		write!(
			formatter,
			"Status: {}",
			self
				.date
				.as_ref()
				.map(InvoiceDate::to_string)
				.unwrap_or_else(|| "Not issued".into())
		)
	}
}

#[cfg(test)]
mod tests
{
	use chrono::{DateTime, Local, Utc};
	use clinvoice_finance::{Currency, Money};
	use pretty_assertions::assert_eq;

	use super::Invoice;
	use crate::InvoiceDate;

	#[test]
	fn display()
	{
		let invoice = Invoice {
			date: Some(InvoiceDate {
				issued: Utc::now(),
				paid: None,
			}),
			hourly_rate: Money::new(10_00, 2, Currency::Usd),
		};

		assert_eq!(
			invoice.to_string(),
			format!(
				"Hourly Rate: 10.00 USD
Status: Issued on {}; Outstanding",
				DateTime::<Local>::from(invoice.date.unwrap().issued),
			),
		);
	}
}
