use thiserror::Error;

/// # Summary
///
/// Errors for the data
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error
{
	/// # Summary
	///
	/// A query was attmepted with regular expressions, and the regular expression was malformed.
	#[cfg_attr(debug_assertions,      error("{0:?}"))]
	#[cfg_attr(not(debug_assertions), error("{0}"))]
	MalformedRegex(#[from] regex::Error),
}

clinvoice_error::AliasResult!();

