use clinvoice_schema::Id;
use serde::{Deserialize, Serialize};

/// Configurations for [`Employee`](clinvoice_schema::Employee)s.
///
/// # Examples
///
/// ```rust
/// use clinvoice_config::Employees;
///
/// let _ = Employees {
///   id: Some(1),
///   organization_id: Some(2),
/// };
/// ```
#[derive(
	Copy, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Employees
{
	/// The [`Id`] of the [`Employee`](clinvoice_schema::Employee) which uses this CLInvoice client.
	///
	/// Frontends for CLInvoice should provide mechanisms to assign this setting for the user.
	pub id: Option<Id>,

	/// The [`Id`] of the [`Organization`](clinvoice_schema::Organization) which uses the CLInvoice
	/// client.
	///
	/// Frontends for CLInvoice should provide mechanisms to assign this setting for the user.
	pub organization_id: Option<Id>,
}
