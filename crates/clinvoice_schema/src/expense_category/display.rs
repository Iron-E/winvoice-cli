use core::fmt::{Display, Formatter, Result};

use super::ExpenseCategory;

impl Display for ExpenseCategory
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{}", self.as_str())
	}
}

#[cfg(test)]
mod tests
{
	use super::ExpenseCategory;

	#[test]
	fn display()
	{
		assert_eq!(format!("{}", ExpenseCategory::Food), "Food");
		assert_eq!(format!("{}", ExpenseCategory::Item), "Item");
		assert_eq!(format!("{}", ExpenseCategory::Other), "Other");
		assert_eq!(format!("{}", ExpenseCategory::Service), "Service");
		assert_eq!(format!("{}", ExpenseCategory::Software), "Software");
		assert_eq!(format!("{}", ExpenseCategory::Travel), "Travel");
	}
}
