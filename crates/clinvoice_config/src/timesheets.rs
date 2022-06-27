mod default;

use core::time::Duration;

use serde::{Deserialize, Serialize};

/// # Summary
///
/// Configurations for [`Timesheet`](clinvoice_schema::timesheet:Timesheet)s.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Timesheets
{
	/// # Summary
	///
	/// The amount of time between increments to the `time_end` on a [`clinvoice_schema::Timesheet`].
	///
	/// # Example
	///
	/// ```rust
	/// let _five_minute_increment = clinvoice_config::Timesheets {
	///   default_increment: core::time::Duration::new(300, 0),
	/// };
	/// ```
	#[serde(with = "humantime_serde")]
	pub default_increment: Duration,
}
