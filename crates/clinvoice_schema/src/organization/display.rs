use core::fmt::{Display, Formatter, Result};

use super::Organization;

impl Display for Organization
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{} @ {}", self.name, self.location)?;

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::Organization;
	use crate::Location;

	#[test]
	fn display()
	{
		let organization = Organization {
			id: 0,
			location: Location {
				id: 0,
				name: "Arizona".into(),
				outer: Some(
					Location {
						id: 0,
						name: "USA".into(),
						outer: Some(
							Location {
								id: 0,
								name: "Earth".into(),
								outer: None,
							}
							.into(),
						),
					}
					.into(),
				),
			},
			name: "Big Old Test".into(),
		};

		assert_eq!(
			organization.to_string(),
			"Big Old Test @ Arizona, USA, Earth"
		);
	}
}
