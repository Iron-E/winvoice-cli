use
{
	crate::{DynResult, io::input},
	clinvoice_adapter::{data::LocationAdapter, Store},
	clinvoice_data::views::{ContactView, LocationView},
	std::{collections::HashMap, fmt::Display, io},
};

/// # Summary
///
/// Show a menu for adding [contact information](clinvoice_data::Contact).
fn add_menu(contact_info: &mut HashMap<String, ContactView>, locations: &[LocationView]) -> input::Result<()>
{
	const ADDRESS: &str = "Address";
	const EMAIL: &str = "Email";
	const PHONE: &str = "Phone";
	const ALL_CONTACT_TYPES: [&str; 3] = [ADDRESS, EMAIL, PHONE];

	fn get_label(entity: impl Display) -> io::Result<String>
	{
		input::text(format!("Please enter a label for \"{}\"", entity))
	}

	match input::select_one(&ALL_CONTACT_TYPES, "Select which type of contact info to add")?
	{
		ADDRESS =>
		{
			let location = input::select_one(&locations, "Select the location to add")?;
			contact_info.insert(get_label(&location)?, ContactView::Address(location));
		}

		EMAIL =>
		{
			let email = input::text("Enter an email address (e.g. `foo@gmail.com`)")?;
			contact_info.insert(get_label(&email)?, ContactView::Email(email));
		}

		PHONE =>
		{
			let phone = input::text("Enter a phone number (e.g. `600-555-5555`)")?;
			contact_info.insert(get_label(&phone)?, ContactView::Phone(phone));
		}

		_ => panic!("Unkown contact type"),
	};

	Ok(())
}

/// # Summary
///
/// Show a menu for creating [contact information](clinvoice_data::Contact).
pub fn creation_menu<'store, L>(store: &'store Store) -> DynResult<'store, HashMap<String, ContactView>> where
	L : LocationAdapter<'store> + 'store,
{
	const ADD: &str = "Add";
	const CONTINUE: &str = "Continue";
	const DELETE: &str = "Delete";
	const EDIT: &str = "Edit";
	const ALL_ACTIONS: [&str; 4] = [ADD, CONTINUE, DELETE, EDIT];

	let mut locations = super::location::retrieve_or_err::<L>(store)?;
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
fn delete_menu(contact_info: &mut HashMap<String, ContactView>) -> input::Result<()>
{
	if !contact_info.is_empty()
	{
		contact_info.remove(&input::select_one(
			&contact_info.keys().cloned().collect::<Vec<String>>(),
			"Select a piece of contact information to remove",
		)?);
	}

	Ok(())
}

/// # Summary
///
/// Show a menu for editing [contact information](clinvoice_data::Contact).
fn edit_menu(contact_info: &mut HashMap<String, ContactView>) -> input::Result<()>
{
	if !contact_info.is_empty()
	{
		let to_edit_key = input::select_one(
			&contact_info.keys().filter(|k|
				matches!(contact_info[*k], ContactView::Email(_) | ContactView::Phone(_))
			).cloned().collect::<Vec<String>>(),
			"Select a piece of contact information to edit.",
		)?;

		match input::edit(None, contact_info[&to_edit_key].clone())
		{
			Ok(edit) => { contact_info.insert(to_edit_key, edit); }
			Err(input::Error::NotEdited) => (),
			Err(e) => return Err(e),
		};
	}

	Ok(())
}
