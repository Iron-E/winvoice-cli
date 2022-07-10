use core::fmt::{Display, Formatter, Result};

use super::Text;

impl<D> Display for Text<D>
where
	D: Display,
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		match self
		{
			Self::Bold(text) => write!(formatter, "**{text}**"),
			Self::Italic(text) => write!(formatter, "*{text}*"),
			Self::Math(text) => write!(formatter, "${text}$"),
		}
	}
}
