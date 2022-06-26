use std::error::Error;

pub type DynResult<T> = Result<T, Box<dyn Error>>;
