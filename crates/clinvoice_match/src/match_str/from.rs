use super::MatchStr;

impl From<&str> for MatchStr<String>
{
	fn from(s: &str) -> Self
	{
		Self::from(s.to_string())
	}
}

impl From<String> for MatchStr<String>
{
	fn from(s: String) -> Self
	{
		Self::EqualTo(s)
	}
}
