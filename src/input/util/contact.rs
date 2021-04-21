use
{
	core::fmt::Display,
	std::{collections::HashMap, io},

	crate::{DynResult, input},

	clinvoice_adapter::{data::LocationAdapter, Store},
	clinvoice_data::views::{ContactView, LocationView},
};

/// # Summary
///
/// Show a menu for adding [contact information](clinvoice_data::Contact).
///
/// # Errors
///
/// Will error whenever [`input::select_one`] or [`input::text`] does.
fn add_menu(contact_info: &mut HashMap<String, ContactView>, locations: &[LocationView]) -> input::Result<()>
{
	const ADDRESS: &str = "Address";
	const EMAIL: &str = "Email";
	const PHONE: &str = "Phone";
	const ALL_CONTACT_TYPES: [&str; 3] = [ADDRESS, EMAIL, PHONE];

	const EXPORT_OPTS: [&str; 2] = ["No", "Yes"];
	const FALSE: &str = EXPORT_OPTS[0];

	/// # Summary
	///
	/// Get whether or not a user wants to export a piece of contact information.
	fn get_export(entity: impl Display) -> io::Result<bool>
	{
		let export = input::select_one(&EXPORT_OPTS, format!("Do you want \"{}\" to be listed when exporting `Job`s?", entity))?;
		Ok(match export
		{
			FALSE => false,
			_ => true,
		})
	}

	/// # Summary
	///
	/// Get what a user wants to call a piece of contact information.
	fn get_label(entity: impl Display) -> io::Result<String>
	{
		input::text(format!("Please enter a label for \"{}\"", entity))
	}

	/// # Summary
	///
	/// Collect necessary pieces of contact information and insert them into the `contact_info`.
	macro_rules! insert
	{
		($variant: ident, $var: ident) =>
		{
			let label = get_label(&$var)?;
			let export = get_export(&$var)?;
			contact_info.insert(label, ContactView::$variant {$var, export});
		};
	}

	match input::select_one(&ALL_CONTACT_TYPES, "Select which type of contact info to add")?
	{
		ADDRESS =>
		{
			let location = input::select_one(&locations, "Select the location to add")?;
			insert!(Address, location);
		}

		EMAIL =>
		{
			let email = input::text("Enter an email address (e.g. `foo@gmail.com`)")?;
			insert!(Email, email);
		}

		PHONE =>
		{
			let phone = input::text("Enter a phone number (e.g. `600-555-5555`)")?;
			insert!(Phone, phone);
		}

		_ => panic!("Unkown contact type"),
	};

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
pub fn creation_menu<'err, L>(store: &Store) -> DynResult<'err, HashMap<String, ContactView>> where
	L : LocationAdapter,
	<L as LocationAdapter>::Error : 'err,
{
	const ADD: &str = "Add";
	const CONTINUE: &str = "Continue";
	const DELETE: &str = "Delete";
	const EDIT: &str = "Edit";
	const ALL_ACTIONS: [&str; 4] = [ADD, CONTINUE, DELETE, EDIT];

	let mut locations = super::location::retrieve_views::<L>(store)?;
	locations.sort_by(|l1, l2| l1.name.cmp(&l2.name));

	let mut contact_info = HashMap::<String, ContactView>::new();

	loop
	{
		match input::select_one(&ALL_ACTIONS, "\nThis is the menu for creating contact information.\nWhat would you like to do?")?
		{
			ADD => add_menu(&mut contact_info, &locations)?,
			CONTINUE => break,
			DELETE => delete_menu(&mut contact_info)?,
			EDIT => edit_menu(&mut contact_info)?,
			_ => panic!("Unknown action"),
		};
	}

	Ok(contact_info)
}

/// # Summary
///
/// Show a menu for deleting [contact information](clinvoice_data::Contact).
///
/// # Errors
///
/// Will error whenever [`input::select_one`] does.
fn delete_menu(contact_info: &mut HashMap<String, ContactView>) -> input::Result<()>
{
	if !contact_info.is_empty()
	{
		contact_info.remove(
			&input::select_one(
				&contact_info.keys().cloned().collect::<Vec<_>>(),
				"Select a piece of contact information to remove",
			)?
		);
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
fn edit_menu(contact_info: &mut HashMap<String, ContactView>) -> input::Result<()>
{
	if !contact_info.is_empty()
	{
		let to_edit_key = input::select_one(
			&contact_info.keys().filter(|k|
				matches!(contact_info[*k], ContactView::Email {email: _, export: _} | ContactView::Phone {phone: _, export: _})
			).cloned().collect::<Vec<_>>(),
			"Select a piece of contact information to edit.",
		)?;

		match input::edit_and_restore(format!("Please edit the {}", to_edit_key), &contact_info[&to_edit_key])
		{
			Ok(edit) => { contact_info.insert(to_edit_key, edit); }
			Err(input::Error::NotEdited) => (),
			Err(e) => return Err(e),
		};
	}

	Ok(())
}
