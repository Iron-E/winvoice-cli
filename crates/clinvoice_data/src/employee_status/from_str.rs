use std::str::FromStr;

use super::EmployeeStatus;
use crate::{FromStrError, FromStrResult};

impl FromStr for EmployeeStatus
{
	type Err = FromStrError;

	fn from_str(s: &str) -> FromStrResult<Self>
	{
		Ok(match s
		{
			"Employed" => Self::Employed,
			"Not employed" => Self::NotEmployed,
			"Representative" => Self::Representative,
			_ => return Err(FromStrError("EmployeeStatus", s.into())),
		})
	}
}
