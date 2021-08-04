use core::fmt::{
	Display,
	Formatter,
	Result,
};

/// # Summary
///
/// Types of text within a Markdown document.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Text<D>
where
	D: Display,
{
	/// # Summary
	///
	/// Bold text.
	///
	/// # Example
	///
	/// ```
	/// use clinvoice_export::markdown::Text::Bold;
	///
	/// assert_eq!(Bold("Something").to_string(), "**Something**");
	/// ```
	Bold(D),

	/// # Summary
	///
	/// Italic text.
	///
	/// # Example
	///
	/// ```
	/// use clinvoice_export::markdown::Text::Italic;
	///
	/// assert_eq!(Italic("Something").to_string(), "*Something*");
	/// ```
	Italic(D),

	/// # Summary
	///
	/// LaTeX formatted text.
	///
	/// # Example
	///
	/// ```
	/// use clinvoice_export::markdown::Text::Math;
	///
	/// assert_eq!(Math("Something").to_string(), "$Something$");
	/// ```
	Math(D),
}

impl<D> Display for Text<D>
where
	D: Display,
{
	/// # Summary
	///
	/// Turn this enumeration representation of Markdown into actual Markdown.
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		match self
		{
			Self::Bold(text) => write!(formatter, "**{}**", text),
			Self::Italic(text) => write!(formatter, "*{}*", text),
			Self::Math(text) => write!(formatter, "${}$", text),
		}
	}
}
