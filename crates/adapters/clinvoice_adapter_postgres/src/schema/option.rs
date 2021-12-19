mod display;

use core::fmt::Display;

pub(crate) struct PostgresOption<T>(pub(crate) Option<T>)
where
	T: Display;
