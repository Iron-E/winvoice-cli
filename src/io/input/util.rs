use
{
	super::DynamicResult,
	clinvoice_adapter::
	{
		data::{Error, LocationAdapter, MatchWhen},
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
	let locations = select_location_or_quit::<L, &str>(store, "Select locations to be part of the contact info.")?;

	let mut contact_info = super::select(
		&locations.into_iter().map(|l| l.into()).collect::<Vec<ContactView>>(),
		"Select locations to be a part of the contact info.",
	)?;

	contact_info.push(ContactView::Email("An email address. E.g. `foo@gmail.com`".into()));
	contact_info.push(ContactView::Phone("A phone number. E.g. `600-555-5555`".into()));

	Ok(super::edit(SerdeWrapper {value: contact_info})?.value.into_iter().map(|c| c.into()).collect())
}

pub fn select_location_or_quit<'pass, 'path, 'user, L, S>(store: Store<'pass, 'path, 'user>, prompt: S) -> DynamicResult<Vec<LocationView>> where
	L : LocationAdapter<'pass, 'path, 'user>,
	S : Into<String>,
{
	let locations = L::retrieve(MatchWhen::Any, MatchWhen::Any, MatchWhen::Any, store)?;

	if locations.is_empty()
	{
		return Err(Error::NoData {entity: stringify!(Location)}.into());
	}

	Ok(super::select(
		&locations.into_iter().try_fold(
			Vec::new(),
			|mut v, l| -> DynamicResult<Vec<LocationView>>
			{
				let result: DynamicResult<LocationView> = l.into();
				v.push(result?);
				Ok(v)
			}
		)?,
		prompt
	)?)
}
