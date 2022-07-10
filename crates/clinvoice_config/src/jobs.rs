mod default;

use core::time::Duration;

use serde::{Deserialize, Serialize};

/// Configurations for [`Job`](clinvoice_schema::Job)s.
///
/// # Examples
///
/// ## TOML
///
/// __Note:__ For more on how to format the `default_increment`, see [`humantime_serde`].
///
/// ```rust
/// # assert!(toml::from_str::<clinvoice_config::Jobs>(r#"
/// default_increment = "15min"
/// # "#).is_ok());
/// ```
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Jobs
{
	/// The default `increment` value on a [`Job`](clinvoice_schema::Job). Supports human-readable
	/// deserialization via [`humantime_serde`].
	#[serde(with = "humantime_serde")]
	pub default_increment: Duration,
}
