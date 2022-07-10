use core::time::Duration;
use std::io;

use clinvoice_finance::Error as FinanceError;
use sqlx::{postgres::types::PgInterval, Error, Result};
#[cfg(test)]
use {lazy_static::lazy_static, sqlx::PgPool};

#[cfg(test)]
pub(super) async fn connect() -> PgPool
{
	lazy_static! {
		static ref URL: String = dotenv::var("DATABASE_URL").unwrap();
	}

	PgPool::connect_lazy(&URL).unwrap()
}

pub(super) fn duration_from(interval: PgInterval) -> Result<Duration>
{
	if interval.months > 0
	{
		return Err(Error::Decode(
			"`PgInterval` could not be decoded into `Duration` because of nonstandard time \
			 measurement `months`"
				.into(),
		));
	}

	const MICROSECONDS_IN_SECOND: u64 = 1000000;
	const NANOSECONDS_IN_MICROSECOND: u32 = 1000;
	const SECONDS_IN_DAY: u64 = 86400;

	// Ignore negative microseconds
	let microseconds: u64 = interval.microseconds.try_into().unwrap_or(0);

	let seconds = microseconds / MICROSECONDS_IN_SECOND;
	let nanoseconds = NANOSECONDS_IN_MICROSECOND *
		u32::try_from(microseconds % MICROSECONDS_IN_SECOND)
			.expect("`u64 % 1000000` should have fit into `u32`");

	Ok(Duration::new(
		seconds +
			u64::try_from(interval.days)
				.map(|days| days * SECONDS_IN_DAY)
				// Ignore negative days
				.unwrap_or(0),
		nanoseconds,
	))
}

/// Map some [error](clinvoice_finance::Error) `e` to an [`Error`].
pub(super) fn finance_err_to_sqlx(e: FinanceError) -> Error
{
	match e
	{
		FinanceError::Decimal(e2) => Error::Decode(e2.into()),
		FinanceError::EcbCsvDecode(_) => Error::Io(io::Error::new(io::ErrorKind::InvalidData, e)),
		FinanceError::Io(e2) => Error::Io(e2),
		FinanceError::Reqwest(e2) => Error::Io(io::Error::new(io::ErrorKind::Other, e2)),
		FinanceError::UnsupportedCurrency(_) => Error::Decode(e.into()),
		FinanceError::Zip(e2) => Error::Io(io::Error::new(io::ErrorKind::InvalidData, e2)),
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::{Duration, PgInterval};

	#[test]
	fn duration_from_interval()
	{
		let test = PgInterval {
			months: 3,
			days: 0,
			microseconds: 0,
		};

		// Ensure that irregular "months" interval cannot be decoded
		assert!(super::duration_from(test).is_err());

		let test = PgInterval {
			months: 0,
			days: 17,
			microseconds: 7076700,
		};

		assert_eq!(
			super::duration_from(test).unwrap(),
			Duration::new(1468807, 76700000)
		);
	}
}
