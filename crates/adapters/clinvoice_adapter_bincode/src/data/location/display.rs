use super::BincodeLocation;
use core::fmt::{Display, Formatter, Result as FmtResult};

impl Display for BincodeLocation<'_, '_, '_, '_>
{
	/// # Summary
	///
	/// Format some given [`Location`] so that all of its [containing outer
	/// `Location`](Location::outer_id)s come before it.
	///
	/// # Example
	///
	/// The below outputs:
	///
	/// > Earth, USA, Arizona
	///
	/// ```no_run
	/// use clinvoice_adapter::{Adapters, data::LocationAdapter, Store};
	/// use clinvoice_adapter_toml::data::BincodeLocation;
	///
	/// let store = Store
	/// {
	/// 	adapter: Adapters::TOML,
	/// 	password: None,
	/// 	path: "/some/path",
	/// 	username: None,
	/// };
	///
	/// let earth = BincodeLocation::create("Earth", store).unwrap();
	/// let usa = earth.create_inner("USA").unwrap();
	/// let arizona = usa.create_inner("Arizona").unwrap();
	/// println!("{}", arizona);
	/// ```
	fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult
	{
		let output = String::from(self.location.name);

		loop
		{
			// TODO
			//
			//	let outer = (
			//		SELECT O
			//		FROM Location L
			//		JOIN Location O ON L.outer_id = O.id;
			//	);
			//
			//	output::push(outer.0.name)
			//
			//	if outer.0.outer_id.is_none() { break; }

			break;
		}

		write!(formatter, "{}", output)
	}
}
