use std::{error::Error, result::Result};

pub type DynResult<T> = Result<T, Box<dyn Error>>;
