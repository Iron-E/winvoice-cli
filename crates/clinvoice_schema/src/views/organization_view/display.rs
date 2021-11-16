use core::fmt::{Display, Formatter, Result};

use super::OrganizationView;

impl Display for OrganizationView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "{} @ {}", self.name, self.location)
	}
}

#[cfg(test)]
mod tests
{
	use super::OrganizationView;
	use crate::views::LocationView;

	#[test]
	fn display()
	{
		let organization = OrganizationView {
			id: 0,
			location: LocationView {
				id:    0,
				name:  "Arizona".into(),
				outer: Some(
					LocationView {
						id:    0,
						name:  "USA".into(),
						outer: Some(
							LocationView {
								id:    0,
								name:  "Earth".into(),
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
			format!("{}", organization),
			"Big Old Test @ Arizona, USA, Earth"
		);
	}
}