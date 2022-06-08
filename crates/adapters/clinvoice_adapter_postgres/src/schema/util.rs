use core::time::Duration;

use clinvoice_finance::Error as FinanceError;
use clinvoice_schema::chrono::{DateTime, SubsecRound, TimeZone};
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

	let (seconds, nanoseconds) = if interval.microseconds > 0
	{
		const MICROSECONDS_IN_SECOND: u64 = 1000000;
		const NANOSECONDS_IN_MICROSECOND: u32 = 1000;
		let microseconds = interval.microseconds as u64;

		(
			microseconds / MICROSECONDS_IN_SECOND,
			(microseconds % MICROSECONDS_IN_SECOND) as u32 * NANOSECONDS_IN_MICROSECOND,
		)
	}
	else
	{
		(0, 0)
	};

	Ok(Duration::new(
		if interval.days > 0
		{
			const SECONDS_IN_DAY: u64 = 86400;
			interval.days as u64 * SECONDS_IN_DAY
		}
		else
		{
			0
		} + seconds,
		nanoseconds,
	))
}

/// # Summary
///
/// Map some [error](clinvoice_finance::Error) `e` to an [`Error`]
pub(super) fn finance_err_to_sqlx(e: FinanceError) -> Error
{
	match e
	{
		FinanceError::Decimal(e2) => Error::Decode(e2.into()),
		FinanceError::EcbCsvDecode(_) | FinanceError::UnsupportedCurrency(_) =>
		{
			Error::Decode(e.into())
		},
		FinanceError::Io(e2) => Error::Io(e2),
		FinanceError::Reqwest(e2) => Error::Protocol(e2.to_string()),
	}
}

/// # Summary
///
/// Ensure that a date time has the correct precision for the postgres database.
pub(super) fn sanitize_datetime<T>(date: DateTime<T>) -> DateTime<T>
where
	T: TimeZone,
{
	date.trunc_subsecs(6)
}

#[cfg(test)]
mod tests
{
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
