use clinvoice_schema::Id;
use serde::{Deserialize, Serialize};

/// # Summary
///
/// Configurations for [`Employee`](clinvoice_schema::employee::Employee)s.
#[derive(
	Copy, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Employees
{
	/// # Summary
	///
	/// The [`Id`] of the [`Employee`](clinvoice_schema::Employee) which uses this CLInvoice client.
	pub id: Option<Id>,

	/// # Summary
	///
	/// The [`Id`] of the [`Organization`](clinvoice_schema::Organization) which the [`Employee`](clinvoice_schema::Employee) which uses this CLInvoice client
	/// works for.
	pub organization_id: Option<Id>,
}
