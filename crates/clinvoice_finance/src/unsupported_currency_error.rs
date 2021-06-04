mod display;
mod error;

/// # Summary
///
/// An [`Error`] to be used whenever a currency is specified by a user which is not supported by
/// CLInvoice.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnsupportedCurrencyError(pub String);
