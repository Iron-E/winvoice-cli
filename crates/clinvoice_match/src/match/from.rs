use core::fmt::Debug;
use std::borrow::Cow::{Borrowed, Owned};

use clinvoice_schema::{Decimal, Money};

use super::Match;

impl<'m, T> From<&'m T> for Match<'m, T>
where
	T: Clone + Debug,
{
	fn from(t: &'m T) -> Self
	{
		Self::EqualTo(Borrowed(t))
	}
}

impl<T> From<T> for Match<'_, T>
where
	T: Clone + Debug,
{
	fn from(t: T) -> Self
	{
		Self::EqualTo(Owned(t))
	}
}
