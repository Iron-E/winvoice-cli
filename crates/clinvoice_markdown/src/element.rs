mod display;

/// # Summary
///
/// Different elements of Markdown.
///
/// # Example
///
/// ```text
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
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Element<T>
{
	/// # Summary
	///
	/// A text block.
	BlockText(T),

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
		depth: usize, text: T
	},

	/// # Summary
	///
	/// A list which ascends in number as the elements.
	///
	/// `depth` is how many preceding `\t` to use.
	OrderedList
	{
		depth: usize, text: T
	},

	/// # Summary
	///
	/// A list which has no inherent order.
	///
	/// `depth` is how many preceding `\t` to use.
	UnorderedList
	{
		depth: usize, text: T
	},
}
