use
{
	crate::{DynResult, io::input},
	super::SerdeWrapper,
	clinvoice_adapter::{data::LocationAdapter, Store},
	clinvoice_data::{Contact, views::ContactView},
	std::collections::HashMap,
};

pub fn edit_select<'store, L>(store: &'store Store) -> DynResult<'store, HashMap<String, Contact>> where
	L : LocationAdapter<'store> + 'store,
{
	let locations = super::location::retrieve_or_err::<L>(store)?;

	fn label(index: usize) -> String
	{
		String::from("[Contact label #") + &index.to_string() + "]"
	}

	let mut contact_info: HashMap<String, ContactView> = input::select(
		&locations.into_iter().map(|l| l.into()).collect::<Vec<ContactView>>(),
		"Select locations to be a part of the contact info.",
	)?.into_iter().enumerate().map(|(index, location)| (label(index), location)).collect();

	contact_info.insert(label(contact_info.len()), ContactView::Email("An email address. E.g. `foo@gmail.com`".into()));
	contact_info.insert(label(contact_info.len()), ContactView::Phone("A phone number. E.g. `600-555-5555`".into()));

	Ok(input::edit(
		Some("Edit your contact information, or add any as necessary."),
		SerdeWrapper {value: contact_info},
	)?.value.into_iter().map(|(label, contact)| (label, contact.into())).collect())
}
