mod display;

use core::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PostgresOption<D>(pub(crate) Option<D>)
where
	D: Display;
