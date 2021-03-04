/// # Summary
///
/// Derives [`From`]`<$from>` for an `Error` type in scope using the `$variant` to determine which
/// kind of `Error` to return.
///
/// # Examples
///
/// See any of the `clinvoice` custom [`Error`] types.
#[macro_export]
macro_rules! FromError
{
	($variant: ident, $from: path) =>
	{
		impl From<$from> for Error
		{
			fn from(err: $from) -> Self
			{
				Self::$variant {err}
			}
		}
	};
}
