mod display;

use core::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PgTypeCast<TCast, TIdent>(pub(crate) TIdent, pub(crate) TCast)
where
	TIdent: Display, TCast: Display;
