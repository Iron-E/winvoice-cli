mod display;

/// # Summary
///
/// Types of text within a Markdown document.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Text<T>
{
	/// # Summary
	///
	/// Bold text.
	Bold(T),

	/// # Summary
	///
	/// Italic text.
	Italic(T),

	/// # Summary
	///
	/// LaTeX formatted text.
	Math(T),
}
