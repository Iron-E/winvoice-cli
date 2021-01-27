mod display;
mod error;

use std::borrow::Cow;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdapterMismatchError<'msg>
{
	pub message: Cow<'msg, str>,
}
