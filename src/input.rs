mod error;
pub mod expense;

use core::{
	fmt::{Debug, Display},
	str::FromStr,
};
use std::io;

use clinvoice_adapter::Retrievable;
use clinvoice_schema::RestorableSerde;
use dialoguer::{Confirm, Editor, Input, MultiSelect, Select};
pub use error::{Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml as yaml;
use sqlx::{Database, Executor, Pool};

use crate::{fmt, DynResult};

/// The prompt for when [matching](clinvoice_match).
const MATCH_PROMPT: &str =
	"See the documentation of this query at https://github.com/Iron-E/clinvoice/wiki/Query-Syntax#";

/// `prompt` the user with a yes/no question.
///
/// # Returns
///
/// * [`Ok(true)`] if the user answers "yes".
/// * [`Ok(false)`] if the user answers "no".
/// * [`Err`] if there was an error gathering input.
pub fn confirm<T>(prompt: T) -> io::Result<bool>
where
	T: Into<String>,
{
	Confirm::new().with_prompt(prompt).interact()
}

/// If a `prompt` is [`confirm`]ed, return `some` value.
pub fn confirm_then_some<Prompt, Some>(prompt: Prompt, some: Some) -> Option<Some>
where
	Prompt: Into<String>,
{
	confirm(prompt).unwrap_or(false).then_some(some)
}

/// Gather input from the user's text editor, defined by the:
///
/// 1. "VISUAL" environment variable.
/// 2. "EDITOR" environment variable.
/// 3. Platform default (Notepad on Windows, Vi on Unix).
pub fn edit<Entity, Prompt>(entity: &Entity, prompt: Prompt) -> Result<Entity>
where
	Entity: DeserializeOwned + Serialize,
	Prompt: AsRef<str>,
{
	let to_edit = yaml::to_string(&entity).map(|serialized| {
		format!("# {}\n\n{serialized}", prompt.as_ref().replace('\n', "\n# "),)
	})?;

	let maybe_edited = Editor::new().extension(".yaml").edit(&to_edit)?;

	maybe_edited.ok_or(Error::NotEdited).and_then(|edit| yaml::from_str(&edit).map_err(Error::from))
}

/// [Edit](edit) an `entity`, and then [restore](clinvoice_schema::RestorableSerde) it.
pub fn edit_and_restore<Entity, Prompt>(entity: &Entity, prompt: Prompt) -> Result<Entity>
where
	Entity: DeserializeOwned + RestorableSerde + Serialize,
	Prompt: AsRef<str>,
{
	let mut edited = edit(entity, prompt)?;
	edited.try_restore(entity)?;
	Ok(edited)
}

/// [Edit](edit) `Entity::default`, returning that `default` if [no edits](Error::NotEdited) were
/// made.
pub fn edit_default<Entity, Prompt>(prompt: Prompt) -> Result<Entity>
where
	Entity: Default + DeserializeOwned + Serialize,
	Prompt: AsRef<str>,
{
	let default = Entity::default();
	edit(&default, prompt).or_else(|e| match e
	{
		Error::NotEdited => Ok(default),
		_ => Err(e),
	})
}

/// [Retrieve](Retrievable::retrieve) all [entities](Retrievable::Entity) that match a
/// user-provided query.
pub async fn retrieve<Retr, Db, Prompt>(
	connection: &Pool<Db>,
	prompt: Prompt,
) -> DynResult<Vec<Retr::Entity>>
where
	Db: Database,
	Prompt: Display,
	Retr: Retrievable<Db = Db>,
	Retr::Match: Default + DeserializeOwned + Serialize,
	for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
{
	loop
	{
		let match_condition: Retr::Match =
			edit_default(format!("{prompt}\n{}locations", MATCH_PROMPT))?;

		let results = Retr::retrieve(connection, match_condition).await?;

		if results.is_empty() &&
			confirm("That query did not return any results, would you like to try again?")?
		{
			continue;
		}

		return Ok(results);
	}
}

/// `prompt` users to select elements from `entities`, returning them.
///
/// TODO: analyze usage to see if `entities` should be `Vec<T>`
pub fn select<Entity, Prompt>(entities: &[Entity], prompt: Prompt) -> io::Result<Vec<Entity>>
where
	Entity: Clone + Display,
	Prompt: Into<String>,
{
	select_indices(entities, prompt)
		.map(|indices| indices.into_iter().map(|i| entities[i].clone()).collect())
}

/// `prompt` users to select elements from `entities`, and then return the index where they appear.
pub fn select_indices<Entity, Prompt>(entities: &[Entity], prompt: Prompt) -> io::Result<Vec<usize>>
where
	Entity: Clone + Display,
	Prompt: Into<String>,
{
	if entities.is_empty()
	{
		return Ok(Vec::new());
	}

	MultiSelect::new().items(entities).with_prompt(prompt).interact()
}

/// `prompt` users to select one element from `entities`, returning it.
///
/// TODO: analyze usage to see if `entities` should be `Vec<T>`
///
/// # Errors
///
/// * When [`select_one_index`] does.
pub fn select_one<Entity, Prompt>(entities: &[Entity], prompt: Prompt) -> Result<Entity>
where
	Entity: Clone + Display,
	Prompt: Into<String>,
{
	select_one_index(entities, prompt).map(|i| entities[i].clone())
}

/// `prompt` users to select one element from `entities`, returning the index where it is found.
///
/// # Errors
///
/// * When `entities` is empty.
/// * When [`Select::interact`] does.
pub fn select_one_index<Entity, Prompt>(entities: &[Entity], prompt: Prompt) -> Result<usize>
where
	Entity: Clone + Display,
	Prompt: Into<String>,
{
	if entities.is_empty()
	{
		return Err(Error::NoData(crate::fmt::type_name::<Entity>().into()));
	}

	// {{{
	let mut s = Select::new();
	s.items(entities).with_prompt(prompt);
	// }}}

	let selector = s;
	loop
	{
		match selector.interact()
		{
			Err(e)
				if e.kind() == io::ErrorKind::Other &&
					e.to_string().contains("Quit not allowed") =>
			{
				println!("Please select something, or press Ctrl+C to quit");
			},
			result => return result.map_err(Error::from),
		};
	}
}

/// [`select_one`] from:
///
/// * If `match_condition` is [`None`], values the user was `prompt`ed to [`retrieve`].
/// * If `match_condition` is [`Some`], values matching the condition.
pub async fn select_one_retrieved<Retr, Db, Prompt>(
	connection: &Pool<Db>,
	match_condition: Option<Retr::Match>,
	prompt: Prompt,
) -> DynResult<Retr::Entity>
where
	Db: Database,
	Prompt: Display,
	Retr: Retrievable<Db = Db>,
	Retr::Entity: Clone + Display,
	Retr::Match: Default + DeserializeOwned + Serialize,
	for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
{
	let retrieved = match match_condition
	{
		Some(condition) => Retr::retrieve(connection, condition).await?,
		_ => retrieve::<Retr, _, _>(connection, prompt).await?,
	};

	#[cfg(test)]
	return Ok(retrieved[0].clone());

	let selected =
		select_one(&retrieved, format!("Select a {}", fmt::type_name::<Retr::Entity>()))?;

	Ok(selected)
}

/// [`select`] from:
///
/// * If `match_condition` is [`None`], values the user was `prompt`ed to [`retrieve`].
/// * If `match_condition` is [`Some`], values matching the condition.
pub async fn select_retrieved<Retr, Db, Prompt>(
	connection: &Pool<Db>,
	match_condition: Option<Retr::Match>,
	prompt: Prompt,
) -> DynResult<Vec<Retr::Entity>>
where
	Db: Database,
	Prompt: Display,
	Retr: Retrievable<Db = Db>,
	Retr::Entity: Clone + Display,
	Retr::Match: Default + DeserializeOwned + Serialize,
	for<'connection> &'connection mut Db::Connection: Executor<'connection, Database = Db>,
{
	let retrieved = match match_condition
	{
		Some(condition) => Retr::retrieve(connection, condition).await?,
		_ => retrieve::<Retr, _, _>(connection, prompt).await?,
	};

	#[cfg(test)]
	return Ok(retrieved);

	let selected = select(&retrieved, format!("Select the {}s", fmt::type_name::<Retr::Entity>()))?;

	Ok(selected)
}

/// `prompt` the user to enter text, and return what they entered.
pub fn text<Text, Prompt>(default_text: Option<Text>, prompt: Prompt) -> io::Result<Text>
where
	Prompt: Into<String>,
	Text: Clone + FromStr + Display,
	Text::Err: Display + Debug,
{
	let mut input = Input::new();

	if let Some(text) = default_text
	{
		input.default(text);
	}

	input.with_prompt(prompt).interact_text()
}
