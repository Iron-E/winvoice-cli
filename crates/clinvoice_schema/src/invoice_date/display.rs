use core::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Local};

use super::InvoiceDate;

impl Display for InvoiceDate
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(
			formatter,
			"Issued on {}; ",
			DateTime::<Local>::from(self.issued)
		)?;

		if let Some(date) = self.paid
		{
			return write!(formatter, "Paid on {}", DateTime::<Local>::from(date));
		}

		write!(formatter, "Outstanding")
	}
}

#[cfg(test)]
mod tests
{
	use chrono::Utc;

	use super::{DateTime, InvoiceDate, Local};

	#[test]
	fn display()
	{
		let date = InvoiceDate {
			issued: Utc::now(),
			paid: None,
		};

		let other_date = InvoiceDate {
			issued: Utc::now(),
			paid: Some(Utc::now()),
		};

		assert_eq!(
			format!("{date}"),
			format!(
				"Issued on {}; Outstanding",
				DateTime::<Local>::from(date.issued),
			)
		);
		assert_eq!(
			format!("{other_date}"),
			format!(
				"Issued on {}; Paid on {}",
				DateTime::<Local>::from(other_date.issued),
				DateTime::<Local>::from(other_date.paid.unwrap()),
			)
		);
	}
}
