mod default;

use core::time::Duration;

use serde::{Deserialize, Serialize};

/// Configurations for [`Job`](clinvoice_schema::Job)s.
///
/// # Examples
///
/// ```rust
/// use core::time::Duration;
/// use clinvoice_config::Jobs;
///
/// let _ = Jobs {
///   default_increment: Duration::from_secs(300),
/// };
/// ```
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Jobs
{
	/// The default `increment` value on a [`Job`](clinvoice_schema::Job). Supports human-readable
	/// deserialization via [`humantime_serde`].
	#[serde(with = "humantime_serde")]
	pub default_increment: Duration,
}
