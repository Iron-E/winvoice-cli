use clinvoice_schema::Id;
use serde::{Deserialize, Serialize};

/// Configurations for [`Employee`](clinvoice_schema::Employee)s.
///
/// # Examples
///
/// ## TOML
///
/// ```rust
/// # assert!(toml::from_str::<clinvoice_config::Employees>(r#"
/// id = 1
/// organization_id = 2
/// # "#).is_ok());
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
