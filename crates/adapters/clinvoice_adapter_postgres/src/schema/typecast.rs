mod display;

use core::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgTypeCast<D>(D, &'static str)
where
	D: Display;

impl<D> PgTypeCast<D>
where
	D: Display,
{
	pub(crate) fn numeric(d: D) -> Self
	{
		Self(d, "numeric")
	}
}
