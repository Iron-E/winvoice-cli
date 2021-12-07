use core::time::Duration;

#[cfg(test)]
use lazy_static::lazy_static;
use sqlx::{
	postgres::{types::PgInterval, PgPool},
	Error,
	Result,
};

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

	let (microseconds_into_secs, microseconds_into_nanos) = if interval.microseconds > 0
	{
		const MICROSECONDS_IN_SECOND: u64 = 1000000;
		const NANOSECONDS_IN_MICROSECOND: u32 = 1000;
		let microseconds = interval.microseconds as u64;

		(
			microseconds.div_euclid(MICROSECONDS_IN_SECOND),
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
		} + microseconds_into_secs,
		microseconds_into_nanos,
	))
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
