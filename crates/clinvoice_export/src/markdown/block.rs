mod display;

/// Syntax for different groupings of text.
///
/// # Examples
///
/// ```rust
/// use core::fmt::Write;
/// use clinvoice_export::markdown::Block;
///
/// let mut expected = String::new();
///
/// writeln!(expected, "{}", Block::Heading {
///   indents: 1,
///   text: "This is a test heading!",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::Heading {
///   indents: 2,
///   text: "Paragraphs",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::Text("I can create a paragraph.")).unwrap();
///
/// writeln!(expected, "{}", Block::Heading {
///   indents: 2,
///   text: "Lists",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::OrderedList {
///   indents: 0,
///   text: "Ordered lists are not a problem.",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::OrderedList {
///   indents: 0,
///   text: "Continuing is just fine.",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::<String>::Break).unwrap();
///
/// writeln!(expected, "{}", Block::UnorderedList {
///   indents: 0,
///   text: "I can break at any point.",
/// })
/// .unwrap();
///
/// writeln!(expected, "{}", Block::UnorderedList {
///   indents: 1,
///   text: "Indenting? Eazy breezy.",
/// })
/// .unwrap();
///
/// write!(expected, "{}", Block::UnorderedList {
///   indents: 0,
///   text: "De-indenting? Easier!",
/// })
/// .unwrap();
///
/// assert_eq!(&expected,
/// "# This is a test heading!
///
/// ### Paragraphs
///
/// I can create a paragraph.
///
/// ### Lists
///
/// 1. Ordered lists are not a problem.
/// 1. Continuing is just fine.
///
/// - I can break at any point.
/// \t- Indenting? Eazy breezy.
/// - De-indenting? Easier!");
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Block<T>
{
	/// A horizontal spacer. Typically used to terminate a list (i.e. [`OrderedList`],
	/// [UnorderedList`]).
	Break,

	/// A heading. `indents` is how many preceding `#`s there are.
	Heading
	{
		indents: usize, text: T
	},

	/// A list which ascends in number with each successive use.
	///
	/// `indents` is how many preceding `\t` to use. Can be used to create sublists.
	OrderedList
	{
		indents: usize, text: T
	},

	/// Plaintext.
	Text(T),

	/// A list which has no inherent order.
	///
	/// `indents` is how many preceding `\t` to use. Can be used to create sublists.
	UnorderedList
	{
		indents: usize, text: T
	},
}
