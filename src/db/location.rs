use super::id::Id;
use core::fmt::{Display, Formatter};
use core::fmt::Result as FmtResult;

/// TODO
pub struct Location<'name>
{
	/// TODO
	_id: Id,
	/// TODO
	_outer_id: Option<Id>,
	/// TODO
	pub name: &'name str,
}

impl<'name> Display for Location<'name>
{
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

		// TODO
		//
		// write!(formatter, output)

		todo!();
	}
}
