use core::fmt::Display;
use std::io;

use clinvoice_adapter::{schema::LocationAdapter, Deletable};
use clinvoice_schema::ContactKind;
use sqlx::{Database, Executor, Pool};

use super::menu;
use crate::{input, DynResult};

/// # Summary
///
/// Show a menu for adding [contact information](clinvoice_schema::Contact).
///
/// # Errors
///
/// Will error whenever [`input::select_one`] or [`input::text`] does.
async fn add_menu<'err, Db, LAdapter>(
	connection: &Pool<Db>,
	contact_info: &mut Vec<(bool, ContactKind, String)>,
) -> DynResult<'err, ()>
where
	Db: Database,
	LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	const ADDRESS: &str = "Address";
	const EMAIL: &str = "Email";
	const PHONE: &str = "Phone";
	const ALL_CONTACT_TYPES: [&str; 3] = [ADDRESS, EMAIL, PHONE];

	/// # Summary
	///
	/// Get whether or not a user wants to export a piece of contact information.
	fn get_export(entity: impl Display) -> input::Result<bool>
	{
		menu::confirm(format!(
			"Do you want \"{entity}\" to be listed when exporting `Job`s?"
		))
	}

	/// # Summary
	///
	/// Get what a user wants to call a piece of contact information.
	fn get_label(entity: impl Display) -> io::Result<String>
	{
		input::text(None, format!("Please enter a label for \"{entity}\""))
	}

	/// # Summary
	///
	/// Collect necessary pieces of contact information and insert them into the `contact_info`.
	macro_rules! insert {
		($variant:ident, $var:ident) => {
			let label = get_label(&$var)?;
			let export = get_export(&$var)?;
			contact_info.push((export, ContactKind::$variant($var), label));
		};
	}

	let contact_type = input::select_one(
		&ALL_CONTACT_TYPES,
		"Select which type of contact info to add",
	)?;

	match contact_type
	{
		ADDRESS =>
		{
			let location = input::util::location::select_one::<&str, _, LAdapter>(
				connection,
				"Query the `Location` which can be used to reach this `Employee`",
				true,
			)
			.await?;
			insert!(Address, location);
		}

		EMAIL =>
		{
			let email = input::text(None, "Enter an email address (e.g. `foo@gmail.com`)")?;
			insert!(Email, email);
		}

		PHONE =>
		{
			let phone = input::text(None, "Enter a phone number (e.g. `600-555-5555`)")?;
			insert!(Phone, phone);
		}

		_ => unreachable!("Unkown contact type. This should not have happened, please file an issue at https://github.com/Iron-E/clinvoice/issues"),
	};

	Ok(())
}
/// # Summary
///
/// Show a menu for deleting [contact information](clinvoice_schema::Contact).
///
/// # Errors
///
/// Will error whenever [`input::select_one`] does.
fn delete_menu(contact_info: &mut Vec<(bool, ContactKind, String)>) -> input::Result<()>
{
	if !contact_info.is_empty()
	{
		let to_remove = input::select_as_indices(
			&contact_info
				.iter()
				.map(|(_, _, label)| label)
				.collect::<Vec<_>>(),
			"Select a contact information to remove",
		)?;

		to_remove.into_iter().for_each(|i| {
			contact_info.remove(i);
		});
	}

	Ok(())
}

/// # Summary
///
/// Show a menu for editing [contact information](clinvoice_schema::Contact).
///
/// # Errors
///
/// Will error whenever [`input::edit_and_restore`] and [`input::select_one`] does,
/// but will ignore [`input::Error::NotEdited`].
async fn edit_menu<'err, Db, LAdapter>(
	connection: &Pool<Db>,
	contact_info: &mut Vec<(bool, ContactKind, String)>,
) -> DynResult<'err, ()>
where
	Db: Database,
	LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	if contact_info.is_empty()
	{
		return Ok(());
	}

	let selected_index = input::select_one_as_index(
		&contact_info
			.iter()
			.map(|(_, _, label)| label)
			.collect::<Vec<_>>(),
		"Select a piece of contact information to edit",
	)?;

	let (selected_contact_export, selected_contact_kind, selected_contact_label) =
		contact_info[selected_index].clone();
	let edited_contact_label = input::text(
		Some(selected_contact_label.clone()),
		format!("Edit the label for \"{selected_contact_kind}\" (optional)"),
	)?;

	let labels_differ = selected_contact_label != edited_contact_label;

	// If the user edited the selected key, we must assert that the new key does not already exist. Otherwise, we will invalidate a constraint on the database that a label be unique per `employee_id`.
	if labels_differ
	{
		// TODO: `if let` chain
		if let Some((_, kind, _)) = contact_info
			.iter()
			.find(|(_, _, label)| label == &edited_contact_label)
		{
			eprintln!("The label \"{edited_contact_label}\" is already being used by \"{kind}\"");
			return Ok(());
		}
	}

	contact_info.push((
		menu::confirm(format!(
			"Do you want \"{edited_contact_label}\" to be listed when exporting `Job`s?"
		))?,
		match selected_contact_kind
		{
			ContactKind::Email(email) =>
			{
				match input::text(
					Some(email),
					format!("Please edit the {selected_contact_label}"),
				)
				{
					Ok(text) => ContactKind::Email(text),
					Err(e) => return Err(e.into()),
				}
			},
			ContactKind::Phone(phone) =>
			{
				match input::text(
					Some(phone),
					format!("Please edit the {selected_contact_label}"),
				)
				{
					Ok(text) => ContactKind::Phone(text),
					Err(e) => return Err(e.into()),
				}
			},
			ContactKind::Address(location) => ContactKind::Address(
				if menu::confirm(format!(
					"Would you like to change the location of {edited_contact_label}? It is currently \
					 {location}."
				))?
				{
					input::util::location::select_one::<&str, _, LAdapter>(
						connection,
						"Query the `Location` which can be used to reach this `Employee`",
						true,
					)
					.await?
				}
				else
				{
					location
				},
			),
		},
		edited_contact_label,
	));

	contact_info.remove(selected_index);
	Ok(())
}

/// # Summary
///
/// Show a menu for creating [contact information](clinvoice_schema::Contact).
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
pub async fn menu<'err, Db, LAdapter>(
	connection: &Pool<Db>,
) -> DynResult<'err, Vec<(bool, ContactKind, String)>>
where
	Db: Database,
	LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	let mut contact_info = Vec::new();

	loop
	{
		let action = input::select_one(
			&menu::ALL_ACTIONS,
			"\nThis is the menu for creating contact information\nWhat would you like to do?",
		)?;
		match action
		{
			menu::ADD => add_menu::<_, LAdapter>(connection, &mut contact_info).await?,
			menu::CONTINUE => break,
			menu::DELETE => delete_menu(&mut contact_info)?,
			menu::EDIT => edit_menu::<_, LAdapter>(connection, &mut contact_info).await?,
			_ => unreachable!("Unknown action. This should not have happened, please file an issue at https://github.com/Iron-E/clinvoice/issues"),
		};
	}

	Ok(contact_info)
}
