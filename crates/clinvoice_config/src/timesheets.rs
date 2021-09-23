mod default;

use core::time::Duration;

use serde::{Deserialize, Serialize};

/// # Summary
///
/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Timesheets
{
	/// # Summary
	///
	/// The amount of time between increments to the [`crate::toml::Timesheet::time_end`] on a timesheet.
	///
	/// # Example
	///
	/// ```rust
	/// use core::time::Duration;
	///
	/// use clinvoice_config::Timesheets;
	///
	/// // 5 minute increment
	/// Timesheets {
	/// 	increment: Duration::new(300, 0),
	/// };
	/// ```
	#[serde(with = "humantime_serde")]
	pub default_increment: Duration,
}
