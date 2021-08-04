//! # Summary
//!
//! This crate provides functionality for tracking and exchanging various [currencies][iso] using
//! the [European Central Bank][ecb].
//!
//! # Features
//!
//! Support for [`serde`](http://serde.rs/) can be enabled with the `serde_support` feature flag.
//! Otherwise, serialization will have to be implemented for these types by hand.
//!
//! [ecb]: https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/
//! [iso]: https://www.iso.org/iso-4217-currency-codes.html

#![allow(clippy::suspicious_else_formatting)]

mod currency;
mod error;
mod exchange_rates;
mod money;

pub use currency::Currency;
pub use error::{
	Error,
	Result,
};
pub use exchange_rates::ExchangeRates;
pub use money::Money;
pub use rust_decimal::Decimal;
