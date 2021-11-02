use super::Contact;
use crate::views::ContactView as View;

impl From<View> for Contact
{
	fn from(view: View) -> Self
	{
		match view
		{
			View::Address { location, export } => Self::Address {
				location_id: location.id,
				export,
			},
			View::Email { email, export } => Self::Email { email, export },
			View::Phone { phone, export } => Self::Phone { phone, export },
		}
	}
}

impl From<&View> for Contact
{
	fn from(view: &View) -> Self
	{
		match view
		{
			View::Address { location, export } => Self::Address {
				location_id: location.id,
				export:      *export,
			},
			View::Email { email, export } => Self::Email {
				email:  email.clone(),
				export: *export,
			},
			View::Phone { phone, export } => Self::Phone {
				phone:  phone.clone(),
				export: *export,
			},
		}
	}
}
