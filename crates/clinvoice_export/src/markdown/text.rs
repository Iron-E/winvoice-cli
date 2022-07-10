mod display;

/// [Markdown](crate::Format::Markdown) syntax for text formatting.
///
/// # Examples
///
/// ```rust
/// use clinvoice_export::markdown::Text;
/// # use pretty_assertions::assert_eq;
///
/// assert_eq!(r#"*I* have a **really strong opinion** about the number $\pi$."#, format!(
///   "{} have a {} about the number {}.",
///   Text::Italic("I"),
///   Text::Bold("really strong opinion"),
///   Text::Math(r#"\pi"#),
/// ));
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Text<T>
{
	/// Bold text.
	Bold(T),

	/// Italic text.
	Italic(T),

	/// Inline LaTeX math.
	Math(T),
}
