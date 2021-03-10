use
{
	crate::{DynResult, io::input},
	super::SerdeWrapper,
	clinvoice_adapter::{data::LocationAdapter, Store},
	clinvoice_data::{Contact, views::ContactView},
};

pub fn select<'store, L>(store: &'store Store) -> DynResult<'store, Vec<Contact>> where
	L : LocationAdapter<'store> + 'store,
{
	let locations = super::location::retrieve_or_err::<L>(store)?;

	let mut contact_info = input::select(
		&locations.into_iter().map(|l| l.into()).collect::<Vec<ContactView>>(),
		"Select locations to be a part of the contact info.",
	)?;

	contact_info.push(ContactView::Email("An email address. E.g. `foo@gmail.com`".into()));
	contact_info.push(ContactView::Phone("A phone number. E.g. `600-555-5555`".into()));

	Ok(input::edit(SerdeWrapper {value: contact_info})?.value.into_iter().map(|c| c.into()).collect())
}
