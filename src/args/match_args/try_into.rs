use std::fs;

use clinvoice_match::{
	MatchContact,
	MatchEmployee,
	MatchExpense,
	MatchJob,
	MatchLocation,
	MatchOrganization,
	MatchTimesheet,
};
use serde_yaml as yaml;

use super::MatchArgs;
use crate::input::{Error, Result};

macro_rules! impl_try_into {
	($T:ty) => {
		impl TryInto<Option<$T>> for MatchArgs
		{
			type Error = Error;

			fn try_into(self) -> Result<Option<$T>>
			{
				let contents = self.r#match.map(fs::read_to_string).transpose()?;
				let deserialized = contents.as_deref().map(yaml::from_str).transpose()?;
				Ok(deserialized)
			}
		}
	};
}

impl_try_into!(MatchContact);
impl_try_into!(MatchEmployee);
impl_try_into!(MatchExpense);
impl_try_into!(MatchJob);
impl_try_into!(MatchLocation);
impl_try_into!(MatchOrganization);
impl_try_into!(MatchTimesheet);
