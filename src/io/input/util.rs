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
};

pub fn contact_info<'pass, 'path, 'user, L>(store: Store<'pass, 'path, 'user>) -> DynamicResult<Vec<Contact>>
	where L : LocationAdapter<'pass, 'path, 'user>
{
	let contact_info = super::select(
		&L::retrieve(MatchWhen::Any, MatchWhen::Any, MatchWhen::Any, store)?.into_iter().try_fold(
			vec![
				ContactView::Email("An email address. E.g. `foo@gmail.com`".into()),
				ContactView::Phone("A phone number. E.g. `600-555-5555`".into()),
			],
			|mut v, l| -> DynamicResult<Vec<ContactView>>
			{
				let result: DynamicResult<LocationView> = l.into();
				v.push(ContactView::Address(result?));
				Ok(v)
			}
		)?,
		"Select locations to be a part of the contact info.",
	)?;

	Ok(super::edit(contact_info)?.into_iter().map(|c| c.into()).collect())
}
