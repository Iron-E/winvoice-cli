use clinvoice_adapter::{schema::ExpensesAdapter, Deletable};
use clinvoice_schema::{Currency, Expense, Id, Money};
use futures::TryFutureExt;
use sqlx::{Database, Executor, Pool};

use crate::{input, DynResult};

/// # Summary
///
/// Show a menu for adding [expenses](clinvoice_schema::Expense).
///
/// # Errors
///
/// Will error whenever [`input::select_one`] or [`input::text`] does.
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
	let category = input::text(None, "What type of `Expense` is this?")?;

	let cost = input::edit(
		&Money::new(20_00, 2, default_currency),
		format!("What is the cost of the {category}?"),
	)?;

	let description = input::edit_markdown(&format!(
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
	loop
	{
		let action = input::select_one(
			&input::menu::ALL_ACTIONS,
			"\nThis is the menu for entering expenses\nWhat would you like to do?",
		)?;

		match action
		{
			input::menu::ADD => add_menu::<_, XAdapter>(connection, expenses, default_currency, timesheet_id).await?,
			input::menu::CONTINUE => return Ok(()),
			input::menu::DELETE => delete_menu::<_, XAdapter>(connection, expenses).await?,
			input::menu::EDIT => edit_menu::<_, XAdapter>(connection, expenses).await?,
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
		let to_remove_indices = input::select_indices(expenses, "Select expenses to remove")?;

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

/// # Summary
///
/// Show a menu for editing [expenses](clinvoice_schema::Expense).
///
/// # Errors
///
/// Will error whenever [`input::edit_and_restore`] and [`input::select_one`] does,
/// but will ignore [`input::Error::NotEdited`].
async fn edit_menu<Db, XAdapter>(connection: &Pool<Db>, expenses: &mut [Expense]) -> DynResult<()>
where
	Db: Database,
	XAdapter: Deletable<Db = Db> + ExpensesAdapter,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	if !expenses.is_empty()
	{
		let edit_index = input::select_one_index(expenses, "Select an expense to edit")?;
		let to_edit = &expenses[edit_index];

		match input::edit(
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
			Err(input::Error::NotEdited) => (),
			Err(e) => return Err(e.into()),
		};
	}

	Ok(())
}
