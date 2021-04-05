mod markdown_element;
mod markdown_text;

pub use
{
	markdown_element::MarkdownElement,
	markdown_text::MarkdownText,
};

/// # Summary
///
/// A trait defining how
pub trait IntoMarkdown
{
	/// # Summary
	///
	/// Convert this element [`MarkdownElement`]s.
	fn into_markdown(self) -> String;
}
