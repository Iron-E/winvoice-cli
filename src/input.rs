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

/// Gather input from the user's text editor, defined by the:
///
/// 1. "VISUAL" environment variable.
/// 2. "EDITOR" environment variable.
/// 3. Platform default (Notepad on Windows, Vi on Unix).
pub fn edit<TEntity, TPrompt>(entity: &TEntity, prompt: TPrompt) -> Result<TEntity>
where
	TEntity: DeserializeOwned + Serialize,
	TPrompt: AsRef<str>,
{
	let to_edit = yaml::to_string(&entity).map(|serialized| {
		format!(
			"# {}\n\n{serialized}",
			prompt.as_ref().replace('\n', "\n# "),
		)
	})?;

	let maybe_edited = Editor::new().extension(".yaml").edit(&to_edit)?;

	maybe_edited
		.ok_or(Error::NotEdited)
		.and_then(|edit| yaml::from_str(&edit).map_err(Error::from))
}

/// [Edit](edit) an `entity`, and then [restore](clinvoice_schema::RestorableSerde) it.
pub fn edit_and_restore<TEntity, TPrompt>(entity: &TEntity, prompt: TPrompt) -> Result<TEntity>
where
	TEntity: DeserializeOwned + RestorableSerde + Serialize,
	TPrompt: AsRef<str>,
{
	let mut edited = edit(entity, prompt)?;
	edited.try_restore(entity)?;
	Ok(edited)
}

/// [Edit](edit) `TEntity::default`, returning that `default` if [no edits](Error::NotEdited) were
/// made.
pub fn edit_default<TEntity, TPrompt>(prompt: TPrompt) -> Result<TEntity>
where
	TEntity: Default + DeserializeOwned + Serialize,
	TPrompt: AsRef<str>,
{
	let default = TEntity::default();
	edit(&default, prompt).or_else(|e| match e
	{
		Error::NotEdited => Ok(default),
		_ => Err(e),
	})
}

/// [Edit](Editor::edit) some `prompt`, rendered as Markdown.
pub fn edit_markdown<T>(content: T) -> Result<String>
where
	T: AsRef<str>,
{
	let result = Editor::new().extension(".md").edit(content.as_ref())?;
	result.ok_or(Error::NotEdited)
}

/// `prompt` users to select elements from `entities`, returning them.
///
/// TODO: analyze usage to see if `entities` should be `Vec<T>`
pub fn select<TEntity, TPrompt>(entities: &[TEntity], prompt: TPrompt) -> io::Result<Vec<TEntity>>
where
	TEntity: Clone + Display,
	TPrompt: Into<String>,
{
	select_indices(entities, prompt)
		.map(|indices| indices.into_iter().map(|i| entities[i].clone()).collect())
}

/// `prompt` users to select elements from `entities`, and then return the index where they appear.
pub fn select_indices<TEntity, TPrompt>(
	entities: &[TEntity],
	prompt: TPrompt,
) -> io::Result<Vec<usize>>
where
	TEntity: Clone + Display,
	TPrompt: Into<String>,
{
	if entities.is_empty()
	{
		return Ok(Vec::new());
	}

	MultiSelect::new()
		.items(entities)
		.paged(true)
		.with_prompt(prompt)
		.interact()
}

/// `prompt` users to select one element from `entities`, returning it.
///
/// TODO: analyze usage to see if `entities` should be `Vec<T>`
///
/// # Errors
///
/// * When [`select_one_index`] does.
pub fn select_one<TEntity, TPrompt>(entities: &[TEntity], prompt: TPrompt) -> Result<TEntity>
where
	TEntity: Clone + Display,
	TPrompt: Into<String>,
{
	select_one_index(entities, prompt).map(|i| entities[i].clone())
}

/// `prompt` users to select one element from `entities`, returning the index where it is found.
///
/// # Errors
///
/// * When `entities` is empty.
/// * When [`Select::interact`] does.
pub fn select_one_index<TEntity, TPrompt>(entities: &[TEntity], prompt: TPrompt) -> Result<usize>
where
	TEntity: Clone + Display,
	TPrompt: Into<String>,
{
	if entities.is_empty()
	{
		return Err(Error::NoData(format!("`{}`", any::type_name::<TEntity>())));
	}

	let selector = {
		let mut s = Select::new();
		s.items(entities).paged(true).with_prompt(prompt);
		s
	};

	loop
	{
		match selector.interact()
		{
			Err(e)
				if e.kind() == io::ErrorKind::Other && e.to_string().contains("Quit not allowed") =>
			{
				println!("Please select something, or press Ctrl+C to quit");
			},
			result => return result.map_err(Error::from),
		};
	}
}

/// `prompt` the user to enter text, and return what they entered.
pub fn text<TText, TPrompt>(default_text: Option<TText>, prompt: TPrompt) -> io::Result<TText>
where
	TPrompt: Into<String>,
	TText: Clone + FromStr + Display,
	<TText as FromStr>::Err: Display + Debug,
{
	let mut input = Input::new();

	if let Some(text) = default_text
	{
		input.default(text);
	}

	input.with_prompt(prompt).interact_text()
}
