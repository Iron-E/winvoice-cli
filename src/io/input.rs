mod error;
pub mod util;

pub use error::Error;

use
{
	clinvoice_adapter::DynamicResult,
	std::{fmt::Display, io},
	dialoguer::{Editor, MultiSelect},
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
pub fn edit<T>(entity: T) -> DynamicResult<T> where
	T : DeserializeOwned + Serialize
{
	// Write the entity to the `temp_path` and then edit that file.
	match toml_editor().edit(&toml::to_string_pretty(&entity)?)?
	{
		Some(edited) => Ok(toml::from_str(&edited)?),
		_ => Err(Error::NotEdited.into()),
	}
}

/// # Summary
///
/// Get a selecion from a list of elements.
///
/// # Remarks
///
/// The user's specified `$selection` environment variable will be opened first, followed by whichever
/// selection is discovered by the [`edit_file`](edit::edit_file) function.
///
/// # Returns
///
/// * The deserialized entity with values filled in by the user.
/// * An [`Error`] encountered while creating, editing, or removing the temporary file.
pub fn select<T>(entities: &[T], prompt: impl Into<String>) -> io::Result<Vec<T>> where
	T : Clone + DeserializeOwned + Display + Serialize
{

	let selection = MultiSelect::new().items(entities).paged(true).with_prompt(prompt).interact()?;

	Ok(entities.iter().enumerate().filter_map(
		|(i, entity)| selection.binary_search(&i).and(Ok(entity.clone())).ok()
	).collect())
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
