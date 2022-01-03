mod display;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PostgresStr<'a>(pub(crate) &'a str);
