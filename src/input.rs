mod error;
pub mod util;

pub use error::{Error, Result};

use
{
	core::fmt::Display,
	std::io,

	clinvoice_data::views::RestorableSerde,

	dialoguer::{Editor, Input, MultiSelect, Select},
	serde::{de::DeserializeOwned, Serialize},
	serde_yaml as yaml,
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
pub fn edit<T>(entity: &T, prompt: impl AsRef<str>) -> Result<T> where
	T : DeserializeOwned + Serialize
{
	let serialized = yaml::to_string(&entity)?;
	let to_edit = format!("# {}\n\n{}", prompt.as_ref(), serialized);

	// Write the entity to the `temp_path` and then edit that file.
	match Editor::new().extension(".yaml").edit(&to_edit)?
	{
		Some(edited) => Ok(yaml::from_str(&edited)?),
		_ => Err(Error::NotEdited),
	}
}

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
pub fn edit_and_restore<T>(entity: &T, prompt: impl AsRef<str>) -> Result<T> where
	T : DeserializeOwned + RestorableSerde + Serialize
{
	let mut edited = edit(entity, prompt)?;
	edited.restore(entity);
	Ok(edited)
}

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
pub fn edit_default<T>(prompt: impl AsRef<str>) -> Result<T> where
	T : Default + DeserializeOwned + Serialize
{
	let default = T::default();
	match edit(&default, prompt)
	{
		Ok(d) => Ok(d),
		Err(Error::NotEdited) => Ok(default),
		Err(e) => Err(e),
	}
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
/// * The selected entities.
/// * An [`Error`] incurred while selecting.
pub fn select<T>(entities: &[T], prompt: impl Into<String>) -> io::Result<Vec<T>> where
	T : Clone + Display
{
	if !entities.is_empty()
	{
		let selection = MultiSelect::new().items(entities).paged(true).with_prompt(prompt).interact()?;

		return Ok(entities.iter().enumerate().filter_map(
			|(i, entity)| match selection.binary_search(&i)
			{
				Ok(_) => Some(entity.clone()),
				_ => None,
			},
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
/// * The selected entity.
/// * An [`Error`] incurred while selecting.
pub fn select_one<T>(entities: &[T], prompt: impl Into<String>) -> io::Result<T> where
	T : Clone + Display
{

	let mut selector = Select::new();
	selector.items(entities).paged(true).with_prompt(prompt);

	loop
	{
		return match selector.interact()
		{
			Ok(index) => Ok(entities[index].clone()),
			Err(e) if e.kind() != io::ErrorKind::Other || !e.to_string().contains("Quit not allowed") => Err(e),
			_ =>
			{
				println!("Please select something, or press Ctrl+C to quit");
				continue
			},
		}
	}
}

/// # Summary
///
/// `prompt` the user to enter text.
pub fn text(prompt: impl Into<String>) -> io::Result<String>
{
	Input::new().with_prompt(prompt).interact_text()
}
