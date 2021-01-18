mod display;
mod error;

use std::borrow::Cow;

#[derive(Debug)]
pub struct AdapterMismatchError<'msg>
{
	pub message: Cow<'msg, str>,
}
