use chrono::Duration;

/// # Summary
///
/// The `TimeConfig` contains settings related to timesheets and time tracking services which are
/// offered by `clinvoice`.
///
/// # Remarks
///
/// The command `clinvoice time` is the scope of this structures fields.
pub struct TimeConfig
{
	/// # Summary
	///
	/// The amount of time between increments to the [`crate::toml::Timesheet::time_end`] on a timesheet.
	///
	/// # Example
	///
	/// ```rust
	/// TimeConfig {interval: Duration::minutes(5)}
	/// ```
	pub interval: Duration,
}
