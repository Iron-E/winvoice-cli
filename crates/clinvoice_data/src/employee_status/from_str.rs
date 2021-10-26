use std::str::FromStr;

use thiserror::Error;

use super::EmployeeStatus;

/// # Summary
///
/// [`Error`](std::error::Error)s referencing [`Store`](crate::Store)s and [`Adapters`].
#[derive(Clone, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
#[error("Using this adapter requires the {0} feature")]
pub struct FromStrError(pub String);

pub type FromStrResult<T> = std::result::Result<T, FromStrError>;

impl FromStr for EmployeeStatus
{
	type Err = FromStrError;

	fn from_str(s: &str) -> FromStrResult<Self>
	{
		if s == Self::Employed.as_str()
		{
			Ok(Self::Employed)
		}
		else if s == Self::NotEmployed.as_str()
		{
			Ok(Self::NotEmployed)
		}
		else if s == Self::Representative.as_str()
		{
			Ok(Self::Representative)
		}
		else
		{
			Err(FromStrError(s.into()))
		}
	}
}
