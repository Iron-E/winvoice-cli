use std::io;

use thiserror::Error;

/// # Summary
///
/// An [`Error`] to be used whenever a currency is specified by a user which is not supported by
/// CLInvoice.
#[derive(Debug, Error)]
pub enum Error
{
	#[error(transparent)]
	Decimal(#[from] rust_decimal::Error),

	#[error("There was an error decoding the exchange rates CSV from the ECB: {0}")]
	EcbCsvDecode(String),

	#[error(transparent)]
	Io(#[from] io::Error),

	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),

	#[error("The {0} currency is not recognized by CLInvoice. Please see https://github.com/Iron-E/clinvoice/wiki/Usage for a list of supported currencies")]
	UnsupportedCurrency(String),
}

clinvoice_error::AliasResult!();
