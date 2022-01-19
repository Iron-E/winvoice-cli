use clinvoice_schema::{Currency, Expense, Money};

use super::menu::{ADD, ALL_ACTIONS, CONTINUE, DELETE, EDIT};
use crate::input;

/// # Summary
///
/// Show a menu for adding [expenses](clinvoice_schema::Expense).
///
/// # Errors
///
/// Will error whenever [`input::select_one`] or [`input::text`] does.
fn add_menu(expenses: &mut Vec<Expense>, default_currency: Currency) -> input::Result<()>
{
	let category = input::text(None, "What type of `Expense` is this?")?;
	let cost = input::edit(
		&Money::new(20_00, 2, default_currency),
		format!("What is the cost of the {category}?"),
	)?;
	let description = input::edit_markdown(&format!(
		"* Describe the {category}\n* All markdown syntax is valid"
	))?;
	Ok(expenses.push(Expense {
		id: Default::default(), // HACK: what should I do here?
		category,
		cost,
		description,
	}))
}

/// # Summary
///
/// Show a menu for creating [expenses](clinvoice_schema::Expense).
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
		let action = input::select_one(
			&ALL_ACTIONS,
			"\nThis is the menu for entering expenses\nWhat would you like to do?",
		)?;
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
/// Show a menu for deleting [expenses](clinvoice_schema::Expense).
///
/// # Errors
///
/// Will error whenever [`input::select_one`] does.
fn delete_menu(expenses: &mut Vec<Expense>) -> input::Result<()>
{
	if !expenses.is_empty()
	{
		let remove = input::select_one(expenses, "Select an expense to remove")?;

		expenses.remove(expenses.iter().enumerate().fold(0, |i, enumeration| {
			if &remove == enumeration.1
			{
				enumeration.0
			}
			else
			{
				i
			}
		}));
	}

	Ok(())
}

/// # Summary
///
/// Show a menu for editing [expenses](clinvoice_schema::Expense).
///
/// # Errors
///
/// Will error whenever [`input::edit_and_restore`] and [`input::select_one`] does,
/// but will ignore [`input::Error::NotEdited`].
fn edit_menu(expenses: &mut Vec<Expense>) -> input::Result<()>
{
	if !expenses.is_empty()
	{
		let edit = input::select_one(expenses, "Select an expense to edit")?;

		let edit_index = expenses.iter().enumerate().fold(0, |i, enumeration| {
			if &edit == enumeration.1
			{
				enumeration.0
			}
			else
			{
				i
			}
		});

		match input::edit(
			&edit,
			format!("Add any changes desired to the {}", edit.category),
		)
		{
			Ok(edited) =>
			{
				expenses[edit_index] = edited;
			},
			Err(input::Error::NotEdited) => (),
			Err(e) => return Err(e),
		};
	}

	Ok(())
}
