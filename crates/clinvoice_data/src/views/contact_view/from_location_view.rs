use super::ContactView;
use crate::views::LocationView as View;

impl From<View> for ContactView
{
	fn from(location: View) -> Self
	{
		Self::Address(location)
	}
}
