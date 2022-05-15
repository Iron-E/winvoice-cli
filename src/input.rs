mod error;
pub mod util;

use core::{
	any,
	fmt::{Debug, Display},
	str::FromStr,
};
use std::io;

use clinvoice_schema::RestorableSerde;
use dialoguer::{Editor, Input, MultiSelect, Select};
pub use error::{Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml as yaml;

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
pub fn edit<T>(entity: &T, prompt: impl AsRef<str>) -> Result<T>
where
	T: DeserializeOwned + Serialize,
{
	let serialized = yaml::to_string(&entity)?;
	let to_edit = format!(
		"# {}\n\n{serialized}",
		prompt.as_ref().replace('\n', "\n# "),
	);

	let result = Editor::new().extension(".yaml").edit(&to_edit)?;
	let edited = result.ok_or(Error::NotEdited)?;
	yaml::from_str(&edited).map_err(|e| e.into())
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
pub fn edit_and_restore<T>(entity: &T, prompt: impl AsRef<str>) -> Result<T>
where
	T: DeserializeOwned + RestorableSerde + Serialize,
{
	let mut edited = edit(entity, prompt)?;
	edited.try_restore(entity)?;
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
pub fn edit_default<T>(prompt: impl AsRef<str>) -> Result<T>
where
	T: Default + DeserializeOwned + Serialize,
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
	let result = Editor::new().extension(".md").edit(prompt)?;
	result.ok_or(Error::NotEdited)
}

/// # Summary
///
/// `prompt` users to select elements from `entities`, then return them.
///
/// # Returns
///
/// * The selected entities.
/// * An [`Error`] incurred while selecting.
pub fn select<T>(entities: &[T], prompt: impl Into<String>) -> io::Result<Vec<T>>
where
	T: Clone + Display,
{
	select_as_indices(entities, prompt)
		.map(|selection| selection.into_iter().map(|i| entities[i].clone()).collect())
}

/// # Summary
///
/// `prompt` users to select elements from `entities`, then return the indices where they were
/// found.
///
/// # Returns
///
/// * The selected entities.
/// * An [`Error`] incurred while selecting.
pub fn select_as_indices<T>(entities: &[T], prompt: impl Into<String>) -> io::Result<Vec<usize>>
where
	T: Clone + Display,
{
	if entities.is_empty()
	{
		return Ok(Vec::new());
	}

	let selection = MultiSelect::new()
		.items(entities)
		.paged(true)
		.with_prompt(prompt)
		.interact()?;

	Ok(selection)
}

/// # Summary
///
/// `prompt` users to select one element from `entities`, then return it.
///
/// # Returns
///
/// * The selected entity.
/// * An [`Error`] incurred while selecting.
pub fn select_one<T>(entities: &[T], prompt: impl Into<String>) -> Result<T>
where
	T: Clone + Display,
{
	let selection = select_one_as_index(entities, prompt)?;
	Ok(entities[selection].clone())
}

/// # Summary
///
/// `prompt` users to select one element from `entities`, then return the index where it was found.
///
/// # Returns
///
/// * The selected entity.
/// * An [`Error`] incurred while selecting.
pub fn select_one_as_index<T>(entities: &[T], prompt: impl Into<String>) -> Result<usize>
where
	T: Clone + Display,
{
	if entities.is_empty()
	{
		return Err(Error::NoData(format!("`{}`", any::type_name::<T>())));
	}

	let selector = {
		let mut s = Select::new();
		s.items(entities).paged(true).with_prompt(prompt);
		s
	};

	loop
	{
		return match selector.interact()
		{
			Ok(index) => Ok(index),
			Err(e)
				if !(e.kind() == io::ErrorKind::Other &&
					e.to_string().contains("Quit not allowed")) =>
			{
				Err(e.into())
			},
			_ =>
			{
				println!("Please select something, or press Ctrl+C to quit");
				continue;
			},
		};
	}
}

/// # Summary
///
/// `prompt` the user to enter text.
pub fn text<S, T>(default_text: Option<T>, prompt: S) -> io::Result<T>
where
	S: Into<String>,
	T: Clone + FromStr + Display,
	T::Err: Display + Debug,
{
	let mut input = Input::new();
	input.with_prompt(prompt);

	if let Some(text) = default_text
	{
		input.default(text);
	}

	input.interact_text()
}
