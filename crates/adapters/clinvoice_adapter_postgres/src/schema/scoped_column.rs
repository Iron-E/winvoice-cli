mod display;

use core::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgScopedColumn<TColumn, TIdent>(pub(crate) TIdent, pub(crate) TColumn)
where
	TColumn: Display,
	TIdent: Display;
