use std::borrow::Cow::{Borrowed, Owned, self};
use super::MatchStr;

impl<'m> From<&'m str> for MatchStr<Cow<'m, str>>
{
	fn from(s: &'m str) -> Self
	{
		Self::EqualTo(Borrowed(s))
	}
}

impl<'m> From<String> for MatchStr<Cow<'_, str>>
{
	fn from(s: String) -> Self
	{
		Self::EqualTo(Owned(s))
	}
}
