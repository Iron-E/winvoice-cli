use
{
	super::Contact,
	crate::views::ContactView as View,
};

impl From<View> for Contact
{
	fn from(view: View) -> Self
	{
		return match view
		{
			View::Address(location) => Self::Address(location.id),
			View::Email(email) => Self::Email(email),
			View::Phone(phone) => Self::Phone(phone),
		};
	}
}

impl From<&View> for Contact
{
	fn from(view: &View) -> Self
	{
		return match view
		{
			View::Address(location) => Self::Address(location.id),
			View::Email(email) => Self::Email(email.clone()),
			View::Phone(phone) => Self::Phone(phone.clone()),
		};
	}
}
