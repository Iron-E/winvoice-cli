use std::path::PathBuf;

use super::MatchArgs;

impl From<Option<PathBuf>> for MatchArgs
{
	fn from(args: Option<PathBuf>) -> Self
	{
		Self { r#match: args }
	}
}
