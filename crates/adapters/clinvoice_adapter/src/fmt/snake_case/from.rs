use core::fmt::Display;

use super::SnakeCase;

impl<T, Unused> From<T> for SnakeCase<T, Unused>
where
	T: Display,
	Unused: Display,
{
	fn from(head: T) -> Self
	{
		Self::Head(head)
	}
}

impl<TLeft, TRight> From<(TLeft, TRight)> for SnakeCase<TLeft, TRight>
where
	TLeft: Display,
	TRight: Display,
{
	fn from(body: (TLeft, TRight)) -> Self
	{
		Self::Body(body.0, body.1)
	}
}
