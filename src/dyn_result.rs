use std::error::Error;

pub type DynResult<'life, T> = Result<T, Box<dyn 'life + Error>>;
