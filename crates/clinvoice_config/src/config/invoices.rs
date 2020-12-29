use clinvoice_data::rusty_money::Iso;

/// # Summary
///
/// Configurations for [`Invoice`](clinvoice_data::invoice::Invoice)s.
#[derive(Debug)]
pub struct Invoices
{
	pub default_currency: Iso,
}
