mod action;

use action::Action;
use clinvoice_adapter::{schema::ExpensesAdapter, Deletable};
use clinvoice_schema::{Currency, Expense, Id, Money};
use strum::IntoEnumIterator;

use super::{Error, Result};
use crate::DynResult;

/// Show a menu for adding [expenses](clinvoice_schema::Expense).
///
/// # Errors
///
/// Will error whenever [`select_one`](super::select_one) or [`text`](super::text) does.
fn add_menu(expenses: &mut Vec<(String, Money, String)>) -> DynResult<()>
{
	let category = super::text(None, "What type of Expense is this?")?;
	let description = super::text(None, format!("Describe the {category} Expense"))?;
	let cost = super::text(
		None,
		format!(
			"What is the cost of the {category} Expense? e.g. {}",
			Money::new(20_00, 2, Default::default()),
		),
	)?;

	expenses.push((category, cost, description));
	Ok(())
}

/// Show a menu for creating [expenses](clinvoice_schema::Expense). Returns a [`Vec`] of tuples
/// with the fields `category`, `cost`, and `description` defined (in that order).
///
/// # Errors
///
/// Will error whenever [`select_one`](super::select_one), [`add_menu`], [`delete_menu`], or [`edit_menu`] does.
pub fn menu() -> DynResult<Vec<(String, Money, String)>>
{
	let all_actions: Vec<_> = Action::iter().collect();
	let mut expenses = Vec::new();

	loop
	{
		let action = super::select_one(
			&all_actions,
			"\nThis is the menu for entering Expenses\nWhat would you like to do?",
		)?;

		match action
		{
			Action::Add => add_menu(&mut expenses)?,
			Action::Continue => return Ok(expenses),
			Action::Delete => delete_menu(&mut expenses)?,
			Action::Edit => edit_menu(&mut expenses)?,
		};
	}
}

/// Show a menu for deleting `expenses`.
fn delete_menu(expenses: &mut Vec<(String, Money, String)>) -> Result<()>
{
	if !expenses.is_empty()
	{
		let to_remove_indices = super::select_indices(
			&expenses.iter().map(tuple_to_string).collect::<Vec<_>>(),
			"Select Expenses to remove",
		)?;

		// PERF: we use `rev` to prevent `expenses` from having to shift so many indexes after each removal
		to_remove_indices.into_iter().rev().for_each(|i| {
			expenses.remove(i);
		});
	}

	Ok(())
}

/// Show a menu for editing `expenses`.
fn edit_menu(expenses: &mut Vec<(String, Money, String)>) -> Result<()>
{
	if !expenses.is_empty()
	{
		let edit_index = super::select_one_index(
			&expenses.iter().map(tuple_to_string).collect::<Vec<_>>(),
			"Select an Expense to edit",
		)?;

		const PROMPT: &str = "Make any desired changes to the ";
		let (mut category, mut cost, mut description) = expenses.remove(edit_index);

		category = super::text(Some(category), format!("{PROMPT} category"))?;
		cost = super::text(Some(cost), format!("{PROMPT} cost"))?;
		description = super::text(Some(description), format!("{PROMPT} description"))?;

		expenses.push((category, cost, description));
	}

	Ok(())
}

/// Converts the yet-created [`Expense`] into a [`String`].
fn tuple_to_string(t: &(String, Money, String)) -> String
{
	format!("{} {} – {}", t.0, t.1, t.2)
}
