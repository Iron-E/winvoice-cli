use
{
	super::DynamicResult,
	clinvoice_adapter::
	{
		data::{LocationAdapter, MatchWhen},
		Store,
	},
	clinvoice_data::
	{
		Contact,
		views::{ContactView, LocationView}
	},
	serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize)]
struct SerdeWrapper<T> { value: T }

pub fn contact_info<'pass, 'path, 'user, L>(store: Store<'pass, 'path, 'user>) -> DynamicResult<Vec<Contact>>
	where L : LocationAdapter<'pass, 'path, 'user>
{
	let mut contact_info = super::select(
		&L::retrieve(MatchWhen::Any, MatchWhen::Any, MatchWhen::Any, store)?.into_iter().try_fold(
			Vec::new(),
			|mut v, l| -> DynamicResult<Vec<ContactView>>
			{
				let result: DynamicResult<LocationView> = l.into();
				v.push(ContactView::Address(result?));
				Ok(v)
			}
		)?,
		"Select locations to be a part of the contact info.",
	)?;

	contact_info.push(ContactView::Email("An email address. E.g. `foo@gmail.com`".into()));
	contact_info.push(ContactView::Phone("A phone number. E.g. `600-555-5555`".into()));

	Ok(super::edit(SerdeWrapper {value: contact_info})?.value.into_iter().map(|c| c.into()).collect())
}
