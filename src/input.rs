mod error;
pub mod expense;

use core::{
	fmt::{Debug, Display},
	str::FromStr,
};
use std::io;

use clinvoice_adapter::Retrievable;
use clinvoice_export::Format;
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
pub fn edit_text<T>(content: T, format: Option<Format>) -> Result<String>
where
	T: AsRef<str>,
{
	let maybe_edit = Editor::new()
		// HACK: have to use closure here
		.extension(format.map(|f| f.extension()).unwrap_or("txt"))
		.edit(content.as_ref())?;

	maybe_edit.ok_or(Error::NotEdited)
}

/// [Retrieve](Retrievable::retrieve) all [entities](Retrievable::Entity) that match a
/// user-provided query.
///
/// If `RETRY_ON_EMPTY`, the query is attempted again when the query returns no results.
pub async fn retrieve<TRetrievable, TDb, TPrompt>(
	connection: &Pool<TDb>,
	prompt: TPrompt,
) -> DynResult<Vec<TRetrievable::Entity>>
where
	TDb: Database,
	TPrompt: Display,
	TRetrievable: Retrievable<Db = TDb>,
	TRetrievable::Match: Default + DeserializeOwned + Serialize,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	loop
	{
		let match_condition: TRetrievable::Match =
			edit_default(format!("{prompt}\n{}locations", MATCH_PROMPT))?;

		let results = TRetrievable::retrieve(connection, &match_condition).await?;

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
		return Err(Error::NoData(crate::fmt::type_name::<TEntity>().into()));
	}

	let selector = {
		let mut s = Select::new();
		s.items(entities).with_prompt(prompt);
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

/// [`select_one`] from:
///
/// * If `match_condition` is [`None`], values the user was `prompt`ed to [`retrieve`].
/// * If `match_condition` is [`Some`], values matching the condition.
pub async fn select_one_retrieved<TRetrievable, TDb, TPrompt>(
	connection: &Pool<TDb>,
	match_condition: Option<TRetrievable::Match>,
	prompt: TPrompt,
) -> DynResult<TRetrievable::Entity>
where
	TDb: Database,
	TPrompt: Display,
	TRetrievable: Retrievable<Db = TDb>,
	TRetrievable::Entity: Clone + Display,
	TRetrievable::Match: Default + DeserializeOwned + Serialize,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let retrieved = match match_condition
	{
		Some(condition) => TRetrievable::retrieve(&connection, &condition).await?,
		_ => retrieve::<TRetrievable, _, _>(connection, prompt).await?,
	};

	let selected = select_one(
		&retrieved,
		format!("Select a {}", fmt::type_name::<TRetrievable::Entity>()),
	)?;

	Ok(selected)
}

/// [`select`] from:
///
/// * If `match_condition` is [`None`], values the user was `prompt`ed to [`retrieve`].
/// * If `match_condition` is [`Some`], values matching the condition.
pub async fn select_retrieved<TRetrievable, TDb, TPrompt>(
	connection: &Pool<TDb>,
	match_condition: Option<TRetrievable::Match>,
	prompt: TPrompt,
) -> DynResult<Vec<TRetrievable::Entity>>
where
	TDb: Database,
	TPrompt: Display,
	TRetrievable: Retrievable<Db = TDb>,
	TRetrievable::Entity: Clone + Display,
	TRetrievable::Match: Default + DeserializeOwned + Serialize,
	for<'c> &'c mut TDb::Connection: Executor<'c, Database = TDb>,
{
	let retrieved = match match_condition
	{
		Some(condition) => TRetrievable::retrieve(&connection, &condition).await?,
		_ => retrieve::<TRetrievable, _, _>(connection, prompt).await?,
	};

	let selected = select(
		&retrieved,
		format!("Select the {}s", fmt::type_name::<TRetrievable::Entity>()),
	)?;

	Ok(selected)
}

/// `prompt` the user to enter text, and return what they entered.
pub fn text<TText, TPrompt>(default_text: Option<TText>, prompt: TPrompt) -> io::Result<TText>
where
	TPrompt: Into<String>,
	TText: Clone + FromStr + Display,
	TText::Err: Display + Debug,
{
	let mut input = Input::new();

	if let Some(text) = default_text
	{
		input.default(text);
	}

	input.with_prompt(prompt).interact_text()
}
