mod display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeCast<TCast, TColumn>(pub TColumn, pub TCast);
