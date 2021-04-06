/// # Summary
///
/// Types of text within a Markdown document.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Text<'text>
{
	/// # Summary
	///
	/// Bold text.
	///
	/// # Example
	///
	/// ```
	/// use clinvoice_data::export::MarkdownText::Bold;
	///
	/// assert_eq!(Bold("Something").render(), "**Something**");
	/// ```
	Bold(&'text str),

	/// # Summary
	///
	/// Italic text.
	///
	/// # Example
	///
	/// ```
	/// use clinvoice_data::export::MarkdownText::Italic;
	///
	/// assert_eq!(Italic("Something").render(), "*Something*");
	/// ```
	Italic(&'text str),

	/// # Summary
	///
	/// LaTeX formatted text.
	///
	/// # Example
	///
	/// ```
	/// use clinvoice_data::export::MarkdownText::Math;
	///
	/// assert_eq!(Math("Something").render(), "$Something$");
	/// ```
	Math(&'text str),
}

impl Text<'_>
{
	/// # Summary
	///
	/// Turn this enumeration representation of Markdown into actual Markdown.
	pub fn render(self) -> String
	{
		match self
		{
			Self::Bold(text) => format!("**{}**", text),
			Self::Italic(text) => format!("*{}*", text),
			Self::Math(text) => format!("${}$", text),
		}
	}
}
