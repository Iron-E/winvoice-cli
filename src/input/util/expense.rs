use
{
	super::menu::{ADD, ALL_ACTIONS, CONTINUE, DELETE, EDIT},
	crate::input,

	clinvoice_data::
	{
		finance::{Currency, Money},
		Expense, ExpenseCategory,
	},
};

/// # Summary
///
/// Show a menu for adding [contact information](clinvoice_data::Contact).
///
/// # Errors
///
/// Will error whenever [`input::select_one`] or [`input::text`] does.
fn add_menu(expenses: &mut Vec<Expense>, default_currency: Currency) -> input::Result<()>
{
	const ALL_EXPENSE_CATEGORIES: [ExpenseCategory; 6] =
	[
		ExpenseCategory::Food,
		ExpenseCategory::Item,
		ExpenseCategory::Other,
		ExpenseCategory::Service,
		ExpenseCategory::Software,
		ExpenseCategory::Travel,
	];

	let category = input::select_one(&ALL_EXPENSE_CATEGORIES, "Select which type of `Expense` to add")?;
	let cost = input::edit(&Money::new(2000, 2, default_currency), format!("What is the cost of the {}?", category))?;
	let description = input::edit_markdown(&format!("* Describe the {}\n* All markdown syntax is valid", category))?;
	expenses.push(Expense {category, cost, description});

	Ok(())
}

/// # Summary
///
/// Show a menu for creating [contact information](clinvoice_data::Contact).
///
/// # Errors
///
/// Will error whenever [`input::select_one`], [`add_menu`], [`delete_menu`], or [`edit_menu`] does.
///
/// # Panics
///
/// If a user manages to select an action (e.g. `ADD`, `CONTINUE`, `DELETE`) which is unaccounted
/// for. This is __theoretically not possible__ but must be present to account for the case of an
/// unrecoverable state of the program.
pub fn menu(expenses: &mut Vec<Expense>, default_currency: Currency) -> input::Result<()>
{
	loop
	{
		let action = input::select_one(&ALL_ACTIONS, "\nThis is the menu for entering expenses\nWhat would you like to do?")?;
		match action
		{
			ADD => add_menu(expenses, default_currency)?,
			CONTINUE => return Ok(()),
			DELETE => delete_menu(expenses)?,
			EDIT => edit_menu(expenses)?,
			_ => unreachable!("Unknown action. This should not have happened, please file an issue at https://github.com/Iron-E/clinvoice/issues"),
		};
	}
}

/// # Summary
///
/// Show a menu for deleting [contact information](clinvoice_data::Contact).
///
/// # Errors
///
/// Will error whenever [`input::select_one`] does.
fn delete_menu(expenses: &mut Vec<Expense>) -> input::Result<()>
{
	if !expenses.is_empty()
	{
		let remove = input::select_one(&expenses, "Select an expense to remove")?;

		expenses.remove(expenses.iter().enumerate().fold(0, |i, enumeration|
			if &remove == enumeration.1 { enumeration.0 }
			else { i }
		));
	}

	Ok(())
}

/// # Summary
///
/// Show a menu for editing [contact information](clinvoice_data::Contact).
///
/// # Errors
///
/// Will error whenever [`input::edit_and_restore`] and [`input::select_one`] does,
/// but will ignore [`input::Error::NotEdited`].
fn edit_menu(expenses: &mut Vec<Expense>) -> input::Result<()>
{
	if !expenses.is_empty()
	{
		let edit = input::select_one(&expenses, "Select an expense to edit")?;

		let edit_index = expenses.iter().enumerate().fold(0, |i, enumeration|
			if &edit == enumeration.1 { enumeration.0 }
			else { i }
		);

		match input::edit(&edit, format!("Add any changes desired to the {}", edit.category))
		{
			Ok(edited) => { expenses[edit_index] = edited; }
			Err(input::Error::NotEdited) => (),
			Err(e) => return Err(e),
		};
	}

	Ok(())
}
