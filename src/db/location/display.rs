use core::fmt::{Display, Formatter};
use core::fmt::Result as FmtResult;
use super::Location;

impl Display for Location<'_>
{
	/// # Summary
	///
	/// Format some given [`Location`] so that all of its [containing outer
	/// `Location`](Location::_outer_id)s come before it.
	///
	/// # Example
	///
	/// The below outputs:
	///
	/// > Earth, USA, Arizona
	///
	/// ```rust
	/// println!("{}", Location::new("Earth").new_inner("USA").new_inner("Arizona"));
	/// ```
	fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult
	{
		let output = String::from(self.name);

		loop
		{
			// TODO
			//
			//	let outer = (
			//		SELECT O
			//		FROM Location L
			//		JOIN Location O ON L._outer_id = O._id;
			//	);
			//
			//	output::push(outer.name)
			//
			//	if outer._outer_id.is_none() { break; }

			break;
		}

		write!(formatter, "{}", output)
	}
}

