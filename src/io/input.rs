use
{
	serde::{de::DeserializeOwned, Serialize},
	std::{fs, error::Error, path::Path},
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
	// Create the temp path for the editor to edit.
	let temp_file_stem = edit::Builder::new().tempfile()?;
	let temp_path_str = format!("{}.toml", temp_file_stem.path().to_string_lossy());
	let temp_path = Path::new(&temp_path_str);

	// Write the entity to the `temp_path` and then edit that file.
	fs::write(temp_path, toml::to_string_pretty(&entity)?)?;
	edit::edit_file(&temp_path)?;

	// Retrieve the input from the user and then remove the temp file.
	let input: T = toml::from_slice(&fs::read(&temp_path)?)?;
	fs::remove_file(temp_path)?;

	return Ok(input);
}
