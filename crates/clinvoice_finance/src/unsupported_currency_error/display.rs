use
{
	core::fmt::{Display, Formatter, Result},

	super::UnsupportedCurrencyError,
};

impl Display for UnsupportedCurrencyError
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "The {} currency is not recognized by CLInvoice. Please see https://github.com/Iron-E/clinvoice/wiki/Usage for a list of supported currencies", self.currency)
	}
}
