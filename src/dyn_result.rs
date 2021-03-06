use std::{error::Error, result::Result};

pub type DynResult<'life, T> = Result<T, Box<dyn Error + 'life>>;
