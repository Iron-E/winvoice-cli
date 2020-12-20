use clinvoice_data::chrono::Duration;

/// # Summary
///
/// Configurations for [`Timesheet`](clinvoice_data::timesheet:Timesheet)s.
pub struct Timesheets
{
	/// # Summary
	///
	/// The amount of time between increments to the [`crate::toml::Timesheet::time_end`] on a timesheet.
	///
	/// # Example
	///
	/// ```rust
	/// config::Timesheets {interval: Duration::minutes(5)}
	/// ```
	pub interval: Duration,
}
