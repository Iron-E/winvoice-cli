mod default;

use clinvoice_data::Id;
use serde::{Deserialize, Serialize};

/// # Summary
///
/// Configurations for [`Employee`](clinvoice_data::employee::Employee)s.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Employees
{
	/// # Summary
	///
	/// The [`Id`] of the employee which should be defaulted to when attaching to a timesheet.
	pub default_id: Option<Id>,
}
