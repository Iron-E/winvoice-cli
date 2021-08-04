use std::io;

use thiserror::Error;

/// # Summary
///
/// An [`Error`] to be used whenever a currency is specified by a user which is not supported by
/// CLInvoice.
#[derive(Debug, Error)]
pub enum Error
{
	#[error("{0}")]
	Decimal(#[from] rust_decimal::Error),

	#[error("{0}")]
	Io(#[from] io::Error),

	#[error("{0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("The {0} currency is not recognized by CLInvoice. Please see https://github.com/Iron-E/clinvoice/wiki/Usage for a list of supported currencies")]
	UnsupportedCurrency(String),

	#[error("{0}")]
	Zip(#[from] zip::result::ZipError),
}

clinvoice_error::AliasResult!();
