/// # Summary
///
/// Configurations for [`Invoice`](clinvoice_data::invoice::Invoice)s.
#[derive(Debug)]
pub struct Invoices<'currency>
{
	pub default_currency: &'currency str,
}
