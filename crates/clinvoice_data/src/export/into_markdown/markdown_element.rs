use std::cmp;

/// # Summary
///
/// Different elements of [`Markdown`].
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
/// 1. this is an OrderedList above a Linebreak
///
/// &nbsp;
///
/// 1. this is another OrderedList below a Linebreak
/// ```
pub enum MarkdownElement<'text>
{
	/// # Summary
	///
	/// A text block.
	BlockText(&'text str),

	/// # Summary
	///
	/// A heading.
	Heading {depth: usize, text: &'text str},

	/// # Summary
	///
	/// A space between two lines.
	Linebreak,

	/// # Summary
	///
	/// A list which ascends in number as the elements
	OrderedList {depth: usize, text: &'text str},

	/// # Summary
	///
	/// A list which has no inherent order.
	UnorderedList {depth: usize, text: &'text str},
}

impl MarkdownElement<'_>
{
	/// # Summary
	///
	/// Turn a [`MarkdownElement`] into a [`String`] which is valid markdown.
	pub fn render(self) -> String
	{
		(match self
		{
			Self::BlockText(text) => text.into(),
			Self::Heading {depth, text} => format!("{} {}", "#".repeat(cmp::max(1, depth)), text),
			Self::Linebreak => "\n&nbsp;".into(),
			Self::OrderedList {depth, text} => format!("{}1. {}", "\t".repeat(depth), text),
			Self::UnorderedList {depth, text} => format!("{}- {}", "\t".repeat(depth), text),
		}) + "\n"
	}
}
