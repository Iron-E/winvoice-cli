mod display;

use core::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PostgresOption<T>(pub(crate) Option<T>)
where
	T: Display;
