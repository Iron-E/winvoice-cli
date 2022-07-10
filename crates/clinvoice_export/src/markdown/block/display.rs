use core::fmt::{Display, Formatter, Result};

use super::Block;

impl<D> Display for Block<D>
where
	D: Display,
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		match self
		{
			Self::Text(text) => writeln!(formatter, "{text}"),
			Self::Break => write!(formatter, ""),
			Self::Heading { indents, text } =>
			{
				writeln!(formatter, "{} {text}", "#".repeat(1.max(*indents)))
			},
			Self::OrderedList { indents, text } =>
			{
				write!(formatter, "{}1. {text}", "\t".repeat(*indents))
			},
			Self::UnorderedList { indents, text } =>
			{
				write!(formatter, "{}- {text}", "\t".repeat(*indents))
			},
		}
	}
}
