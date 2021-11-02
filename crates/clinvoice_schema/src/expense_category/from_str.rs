use core::str::FromStr;

use super::ExpenseCategory;
use crate::{FromStrError, FromStrResult};

impl FromStr for ExpenseCategory
{
	type Err = FromStrError;

	fn from_str(s: &str) -> FromStrResult<Self>
	{
		Ok(match s
		{
			"Food" => ExpenseCategory::Food,
			"Item" => ExpenseCategory::Item,
			"Other" => ExpenseCategory::Other,
			"Service" => ExpenseCategory::Service,
			"Software" => ExpenseCategory::Software,
			"Travel" => ExpenseCategory::Travel,
			_ => return Err(FromStrError("ExpenseCategory", s.into())),
		})
	}
}
