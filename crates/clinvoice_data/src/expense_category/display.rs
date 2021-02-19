use
{
	super::ExpenseCategory,
	std::fmt::{Display, Formatter, Result},
};

impl Display for ExpenseCategory
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		return write!(formatter, "{}", match self
		{
			ExpenseCategory::Food => "Food",
			ExpenseCategory::Hosting => "Hosting",
			ExpenseCategory::Item => "Item",
			ExpenseCategory::Other => "Other",
			ExpenseCategory::Software => "Software",
			ExpenseCategory::Travel => "Travel",
		});
	}
}

