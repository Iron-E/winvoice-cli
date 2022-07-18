use clinvoice_adapter::{schema::ExpensesAdapter, Deletable};
use clinvoice_schema::{Currency, Expense, Id, Money};
use futures::TryFutureExt;
use sqlx::{Database, Executor, Pool};

use crate::DynResult;

/// Show a menu for adding [expenses](clinvoice_schema::Expense).
///
/// # Errors
///
/// Will error whenever [`select_one`](super::select_one) or [`text`](super::text) does.
async fn add_menu<Db, XAdapter>(
	connection: &Pool<Db>,
	expenses: &mut Vec<Expense>,
	default_currency: Currency,
	timesheet_id: Id,
) -> DynResult<()>
where
	Db: Database,
	XAdapter: Deletable<Db = Db> + ExpensesAdapter,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	let category = super::text(None, "What type of `Expense` is this?")?;

	let cost = super::edit(
		&Money::new(20_00, 2, default_currency),
		format!("What is the cost of the {category}?"),
	)?;

	let description = super::edit_markdown(&format!(
		"* Describe the {category}\n* All markdown syntax is valid"
	))?;

	if let Some(expense) = XAdapter::create(
		connection,
		vec![(category, cost, description)],
		timesheet_id,
	)
	.await?
	.into_iter()
	.next()
	{
		expenses.push(expense);
	}

	Ok(())
}

/// Show a menu for creating [expenses](clinvoice_schema::Expense).
///
/// # Errors
///
/// Will error whenever [`select_one`](super::select_one), [`add_menu`], [`delete_menu`], or [`edit_menu`] does.
///
/// # Panics
///
/// If a user manages to select an action (e.g. `ADD`, `CONTINUE`, `DELETE`). This is __theoretically
/// not possible__ but must be accounted for because it results in unrecoverable program state.
pub async fn menu<Db, XAdapter>(
	connection: &Pool<Db>,
	expenses: &mut Vec<Expense>,
	default_currency: Currency,
	timesheet_id: Id,
) -> DynResult<()>
where
	Db: Database,
	XAdapter: Deletable<Db = Db> + ExpensesAdapter,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	const ADD: &str = "Add";
	const CONTINUE: &str = "Continue";
	const DELETE: &str = "Delete";
	const EDIT: &str = "Edit";

	loop
	{
		let action = super::select_one(
			&[ADD, CONTINUE, DELETE, EDIT],
			"\nThis is the menu for entering expenses\nWhat would you like to do?",
		)?;

		match action
		{
			ADD => add_menu::<_, XAdapter>(connection, expenses, default_currency, timesheet_id).await?,
			CONTINUE => return Ok(()),
			DELETE => delete_menu::<_, XAdapter>(connection, expenses).await?,
			EDIT => edit_menu::<_, XAdapter>(connection, expenses).await?,
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
/// Will error whenever [`select_one`](super::select_one) does.
async fn delete_menu<Db, XAdapter>(
	connection: &Pool<Db>,
	expenses: &mut Vec<Expense>,
) -> DynResult<()>
where
	Db: Database,
	XAdapter: Deletable<Db = Db> + ExpensesAdapter,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	if !expenses.is_empty()
	{
		let to_remove_indices = super::select_indices(expenses, "Select expenses to remove")?;

		XAdapter::delete(
			connection,
			to_remove_indices
				.into_iter()
				.rev() // PERF: we use `rev` to prevent `expenses` from having to shift so many indexes after each removal
				.map(|i| expenses.remove(i))
				.collect::<Vec<_>>()
				.iter(),
		)
		.await?;
	}

	Ok(())
}

/// Show a menu for editing [expenses](clinvoice_schema::Expense).
///
/// # Errors
///
/// Will error whenever [`edit_and_restore`](super::edit_and_restore) and
/// [`select_one`](super::select_one) does, but will ignore
/// [`Error::NotEdited`](super::Error::NotEdited).
async fn edit_menu<Db, XAdapter>(connection: &Pool<Db>, expenses: &mut [Expense]) -> DynResult<()>
where
	Db: Database,
	XAdapter: Deletable<Db = Db> + ExpensesAdapter,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	if !expenses.is_empty()
	{
		let edit_index = super::select_one_index(expenses, "Select an expense to edit")?;
		let to_edit = &expenses[edit_index];

		match super::edit(
			to_edit,
			format!("Add any changes desired to the {}", to_edit.category),
		)
		{
			Ok(edited) =>
			{
				connection
					.begin()
					.and_then(|mut transaction| async {
						XAdapter::update(&mut transaction, [&edited].into_iter()).await?;
						transaction.commit().await
					})
					.await?;
				expenses[edit_index] = edited;
			},
			Err(super::Error::NotEdited) => (),
			Err(e) => return Err(e.into()),
		};
	}

	Ok(())
}
