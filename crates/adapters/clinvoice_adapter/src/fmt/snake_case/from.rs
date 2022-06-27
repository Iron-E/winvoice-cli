use core::fmt::Display;

use super::SnakeCase;

impl<T> From<T> for SnakeCase<T, &'static str>
where
	T: Display,
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
