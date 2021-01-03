use core::fmt::{Display, Formatter, Result as FmtResult};
use super::TomlLocation;

impl Display for TomlLocation<'_>
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
	/// use clinvoice_adapter::{Adapter, data::CrudLocation};
	/// use clinvoice_adapter_toml::data::TomlLocation;
	///
	/// println!("{}", TomlLocation::insert("Earth").insert_inner("USA").insert_inner("Arizona"));
	/// ```
	fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult
	{
		let output = String::from(self.0.name);

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
