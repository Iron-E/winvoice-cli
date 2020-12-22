use crate::Adapters;

pub struct Connection<'url>
{
	pub adapter: Adapters,
	pub url: &'url str,
}
