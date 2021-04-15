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
pub enum Element<'text>
{
	/// # Summary
	///
	/// A text block.
	BlockText(&'text str),

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
	Heading {depth: usize, text: &'text str},

	/// # Summary
	///
	/// A list which ascends in number as the elements.
	///
	/// `depth` is how many preceding `\t` to use.
	OrderedList {depth: usize, text: &'text str},

	/// # Summary
	///
	/// A list which has no inherent order.
	///
	/// `depth` is how many preceding `\t` to use.
	UnorderedList {depth: usize, text: &'text str},
}

impl Element<'_>
{
	/// # Summary
	///
	/// Turn a [`MarkdownElement`] into a [`String`] which is valid markdown.
	pub fn render(self) -> String
	{
		(match self
		{
			Self::BlockText(text) => format!("{}\n", text),
			Self::Break => String::with_capacity(1),
			Self::Heading {depth, text} => format!("{} {}\n", "#".repeat(1.max(depth)), text),
			Self::OrderedList {depth, text} => format!("{}1. {}", "\t".repeat(depth), text),
			Self::UnorderedList {depth, text} => format!("{}- {}", "\t".repeat(depth), text),
		}) + "\n"
	}
}

#[cfg(test)]
mod tests
{
	use super::Element;

	#[test]
	fn render()
	{
		assert_eq!(vec![
			Element::Heading {depth: 1, text: "This is a test heading!"}.render(),
			Element::Heading {depth: 2, text: "Paragraphs"}.render(),
			Element::BlockText("I can create a paragraph.").render(),
			Element::Heading {depth: 2, text: "Ordered Lists"}.render(),
			Element::OrderedList {depth: 0, text: "Ordered lists are not a problem."}.render(),
			Element::OrderedList {depth: 0, text: "Continuing is just fine."}.render(),
			Element::Break.render(),
			Element::Heading {depth: 2, text: "Break"}.render(),
			Element::Heading {depth: 2, text: "Unordered List"}.render(),
			Element::UnorderedList {depth: 0, text: "I can break at any point."}.render(),
			Element::UnorderedList {depth: 1, text: "Indenting? Eazy breezy."}.render(),
			Element::UnorderedList {depth: 0, text: "De-indenting? Easier!"}.render(),
		].join(""),
"# This is a test heading!

## Paragraphs

I can create a paragraph.

## Ordered Lists

1. Ordered lists are not a problem.
1. Continuing is just fine.

## Break

## Unordered List

- I can break at any point.
	- Indenting? Eazy breezy.
- De-indenting? Easier!
"
		);
	}
}
