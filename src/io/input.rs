use
{
	dialoguer::Editor,
	serde::{de::DeserializeOwned, Serialize},
	std::error::Error,
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
pub fn from_editor<T>(entity: T) -> Result<T, Box<dyn Error>> where
	T : DeserializeOwned + Serialize
{
	// Write the entity to the `temp_path` and then edit that file.
	return Ok(match toml_editor().edit(&toml::to_string_pretty(&entity)?)?
	{
		Some(edited) => toml::from_str(&edited)?,
		None => entity,
	});
}

/// # Summary
///
/// Creates an instance of [`Editor`] which is configured to edit [`toml`] files.
fn toml_editor() -> Editor
{
	let mut editor = Editor::new();
	editor.extension(".toml");
	return editor;
}
