use core::fmt::Display;
use std::{collections::HashMap, io};

use clinvoice_adapter::{schema::LocationAdapter, Deletable};
use clinvoice_schema::views::ContactView;
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
	contact_info: &mut HashMap<String, ContactView>,
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
			"Do you want \"{}\" to be listed when exporting `Job`s?",
			entity
		))
	}

	/// # Summary
	///
	/// Get what a user wants to call a piece of contact information.
	fn get_label(entity: impl Display) -> io::Result<String>
	{
		input::text(None, format!("Please enter a label for \"{}\"", entity))
	}

	/// # Summary
	///
	/// Collect necessary pieces of contact information and insert them into the `contact_info`.
	macro_rules! insert {
		($variant:ident, $var:ident) => {
			let label = get_label(&$var)?;
			let export = get_export(&$var)?;
			contact_info.insert(label, ContactView::$variant { $var, export });
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
			let locations = input::util::location::retrieve_view::<&str, _, LAdapter>(
				connection,
				"Query the `Location` which can be used to reach this `Employee`",
				true,
			).await?;

			let location = input::select_one(&locations, "Select the location to add")?;
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
fn delete_menu(contact_info: &mut HashMap<String, ContactView>) -> input::Result<()>
{
	if !contact_info.is_empty()
	{
		contact_info.remove(&input::select_one(
			&contact_info.keys().cloned().collect::<Vec<_>>(),
			"Select a piece of contact information to remove",
		)?);
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
fn edit_menu(contact_info: &mut HashMap<String, ContactView>) -> input::Result<()>
{
	if contact_info.is_empty()
	{
		return Ok(());
	}

	let selected_key = input::select_one(
		&contact_info.keys().cloned().collect::<Vec<_>>(),
		"Select a piece of contact information to edit",
	)?;
	let typed_key = input::text(
		Some(selected_key.clone()),
		format!(
			"Edit the label for \"{}\" (optional)",
			contact_info[&selected_key]
		),
	)?;
	let keys_differ = selected_key != typed_key;

	/* This section is a little complicated, so there is some annotation to explain what is happening. */

	// If the user edited the selected key, it must be that the new key does not already exist.
	if keys_differ && contact_info.contains_key(&typed_key)
	{
		eprintln!(
			"The label \"{}\" is already being used by \"{}\"",
			typed_key, contact_info[&typed_key]
		);
		return Ok(());
	}

	// We allow users to edit email addresses and phone numebrs during this process, but not addresses.
	// Users can only ever relabel an address, thus we have to gate addresses for below.
	if matches!(
		contact_info[&selected_key],
		ContactView::Email {
			email:  _,
			export: _,
		} | ContactView::Phone {
			phone:  _,
			export: _,
		}
	)
	{
		match input::edit_and_restore(
			&contact_info[&selected_key],
			format!("Please edit the {}", selected_key),
		)
		{
			Ok(edit) => contact_info.insert(typed_key, edit),
			Err(input::Error::NotEdited) => None,
			Err(e) => return Err(e),
		};
	}
	// This check must come after, because the keys could differ but not be an `Address`.
	// Further, we want an `else if` to avoid an unecessary clone of `typed_key`.
	else if keys_differ
	// `&& let`, but that syntax isn't available yet
	{
		if let ContactView::Address { location, export } = contact_info[&selected_key].clone()
		{
			contact_info.insert(typed_key, ContactView::Address { location, export });
		}
	}

	// Finally we have to check _again_ if the keys differ, so that we can remove the old key if need-be.
	if keys_differ
	{
		contact_info.remove(&selected_key);
	}

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
) -> DynResult<'err, HashMap<String, ContactView>>
where
	Db: Database,
	LAdapter: Deletable<Db = Db> + LocationAdapter + Send,
	for<'c> &'c mut Db::Connection: Executor<'c, Database = Db>,
{
	let mut contact_info = HashMap::<String, ContactView>::new();

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
			menu::EDIT => edit_menu(&mut contact_info)?,
			_ => unreachable!("Unknown action. This should not have happened, please file an issue at https://github.com/Iron-E/clinvoice/issues"),
		};
	}

	Ok(contact_info)
}
