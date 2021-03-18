mod error;
pub mod util;

pub use error::{Error, Result};

use
{
	clinvoice_data::views::RestorableSerde,
	std::{fmt::Display, io},
	dialoguer::{Editor, Input, MultiSelect, Select},
	serde::{de::DeserializeOwned, Serialize},
};

/// # Summary
///
/// Gather input from the user's text editor of choice.
///
/// # Remarks
///
/// The user's specified `$EDITOR` environment variable will be opened first, followed by whichever
/// editor is discovered by the [`edit_file`](edit::edit_file) function.
///
/// # Returns
///
/// * The deserialized entity with values filled in by the user.
/// * An [`Error`] encountered while creating, editing, or removing the temporary file.
pub fn edit<T>(prompt: Option<&str>, entity: T) -> Result<T> where
	T : DeserializeOwned + RestorableSerde + Serialize
{
	let serialized = toml::to_string_pretty(&entity)?;
	let to_edit = match prompt
	{
		Some(p) => format!("# {}\n\n{}", p, serialized),
		_ => serialized,
	};

	// Write the entity to the `temp_path` and then edit that file.
	let mut edited: T = match toml_editor().edit(&to_edit)?
	{
		Some(edited) => toml::from_str(&edited)?,
		_ => return Err(Error::NotEdited),
	};
	edited.restore(&entity);
	Ok(edited)
}

/// # Summary
///
/// [Edit](edit_func) markdown based on some `prompt` which will appear in the user's editor.
///
/// # Errors
///
/// * [`io::Error`] when the [edit][edit_func] fails.
/// * [`Error::NotEdited`] when the user does not change the `prompt`.
///
/// [edit_func]: Editor::edit
pub fn edit_markdown(prompt: &str) -> Result<String>
{
	match Editor::new().extension(".md").edit(prompt)?
	{
		Some(edited) => Ok(edited),
		_ => Err(Error::NotEdited),
	}
}

/// # Summary
///
/// `prompt` users to select elements from `entities`, then return them.
///
/// # Returns
///
/// * The deserialized entity with values filled in by the user.
/// * An [`Error`] encountered while creating, editing, or removing the temporary file.
pub fn select<T>(entities: &[T], prompt: impl Into<String>) -> io::Result<Vec<T>> where
	T : Clone + Display
{
	if !entities.is_empty()
	{
		let selection = MultiSelect::new().items(entities).paged(true).with_prompt(prompt).interact()?;

		return Ok(entities.iter().enumerate().filter_map(
			|(i, entity)| selection.binary_search(&i).and(Ok(entity.clone())).ok()
		).collect());
	}

	Ok(Vec::new())
}

/// # Summary
///
/// `prompt` users to select one element from `entities`, then return it.
///
/// # Returns
///
/// * The deserialized entity with values filled in by the user.
/// * An [`Error`] encountered while creating, editing, or removing the temporary file.
pub fn select_one<T>(entities: &[T], prompt: impl Into<String>) -> io::Result<T> where
	T : Clone + Display
{

	let selection = Select::new().items(entities).paged(true).with_prompt(prompt).interact()?;

	Ok(entities[selection].clone())
}

/// # Summary
///
/// `prompt` the user to enter text.
pub fn text(prompt: impl Into<String>) -> io::Result<String>
{
	Input::new().with_prompt(prompt).interact_text()
}

/// # Summary
///
/// Creates an instance of [`Editor`] which is configured to edit [`toml`] files.
pub fn toml_editor() -> Editor
{
	let mut editor = Editor::new();
	editor.extension(".toml");
	editor
}
