use core::fmt::{Display, Formatter, Result};

/// # Summary
///
/// Different elements of Markdown.
///
/// # Example
///
/// ```ignore
/// # this is a Heading
///
/// this is BlockText
///
/// * this is an UnorderedList
///
/// this is another BlockText
///
/// 1. this is an OrderedList, above a Break.
///
/// 1. this is another OrderedList, below a Break.
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Element<D>
where
	D: Display,
{
	/// # Summary
	///
	/// A text block.
	BlockText(D),

	/// # Summary
	///
	/// A space between two [`MarkdownElement`]s.
	///
	/// # Remarks
	///
	/// Typically used on [`OrderedList`]s or [`UnorderedList`]s.
	Break,

	/// # Summary
	///
	/// A heading. `depth` is how many preceding `#`s there are.
	Heading
	{
		depth: usize, text: D
	},

	/// # Summary
	///
	/// A list which ascends in number as the elements.
	///
	/// `depth` is how many preceding `\t` to use.
	OrderedList
	{
		depth: usize, text: D
	},

	/// # Summary
	///
	/// A list which has no inherent order.
	///
	/// `depth` is how many preceding `\t` to use.
	UnorderedList
	{
		depth: usize, text: D
	},
}

impl<D> Display for Element<D>
where
	D: Display,
{
	/// # Summary
	///
	/// Turn a [`MarkdownElement`] into a [`String`] which is valid markdown.
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		match self
		{
			Self::BlockText(text) => writeln!(formatter, "{}", text),
			Self::Break => write!(formatter, ""),
			Self::Heading { depth, text } =>
			{
				writeln!(formatter, "{} {}", "#".repeat(1.max(*depth)), text)
			},
			Self::OrderedList { depth, text } =>
			{
				write!(formatter, "{}1. {}", "\t".repeat(*depth), text)
			},
			Self::UnorderedList { depth, text } =>
			{
				write!(formatter, "{}- {}", "\t".repeat(*depth), text)
			},
		}
	}
}

#[cfg(test)]
mod tests
{
	use core::fmt::Write;

	use super::Element;

	#[test]
	fn fmt()
	{
		let mut expected = String::new();

		assert!(writeln!(expected, "{}", Element::Heading {
			depth: 1,
			text: "This is a test heading!",
		})
		.is_ok());
		assert!(writeln!(expected, "{}", Element::Heading {
			depth: 2,
			text: "Paragraphs",
		})
		.is_ok());
		assert!(writeln!(
			expected,
			"{}",
			Element::BlockText("I can create a paragraph.")
		)
		.is_ok());
		assert!(writeln!(expected, "{}", Element::Heading {
			depth: 2,
			text: "Lists",
		})
		.is_ok());
		assert!(writeln!(expected, "{}", Element::OrderedList {
			depth: 0,
			text: "Ordered lists are not a problem.",
		})
		.is_ok());
		assert!(writeln!(expected, "{}", Element::OrderedList {
			depth: 0,
			text: "Continuing is just fine.",
		})
		.is_ok());
		assert!(writeln!(expected, "{}", Element::<String>::Break).is_ok());
		assert!(writeln!(expected, "{}", Element::UnorderedList {
			depth: 0,
			text: "I can break at any point.",
		})
		.is_ok());
		assert!(writeln!(expected, "{}", Element::UnorderedList {
			depth: 1,
			text: "Indenting? Eazy breezy.",
		})
		.is_ok());
		assert!(write!(expected, "{}", Element::UnorderedList {
			depth: 0,
			text: "De-indenting? Easier!",
		})
		.is_ok());

		assert_eq!(
			expected,
			"# This is a test heading!

## Paragraphs

I can create a paragraph.

## Lists

1. Ordered lists are not a problem.
1. Continuing is just fine.

- I can break at any point.
	- Indenting? Eazy breezy.
- De-indenting? Easier!"
		);
	}
}
