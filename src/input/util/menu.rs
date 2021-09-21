use crate::input;

pub const ADD: &str = "Add";
pub const CONTINUE: &str = "Continue";
pub const DELETE: &str = "Delete";
pub const EDIT: &str = "Edit";
pub const ALL_ACTIONS: [&str; 4] = [ADD, CONTINUE, DELETE, EDIT];

/// # Summary
///
/// Raise a menu asking if the user would like to retry a query.
///
/// # Returns
///
/// * `Ok(true)` if the user wants to retry.
/// * `Ok(false)` if the user does not want to retry.
/// * `Err(_)` if there was an error gathering input.
pub fn ask_to_retry() -> input::Result<bool>
{
	confirm("That query did not return any results, would you like to try again?")
}

/// # Summary
///
/// `prompt` the user with a yes/no question and map the response to a `bool`.
///
/// # Returns
///
/// * `Ok(true)` if the user answers "yes".
/// * `Ok(false)` if the user answers "no".
/// * `Err(_)` if there was an error gathering input.
pub fn confirm(prompt: impl Into<String>) -> input::Result<bool>
{
	const NO: &str = "No";
	const YES: &str = "Yes";
	const OPTIONS: [&str; 2] = [YES, NO];

	input::select_one(&OPTIONS, prompt).map(|confirmed| match confirmed
	{
		YES => true,
		NO => false,
		_ => unreachable!("Unrecognized confirmation. Please file an issue at https://github.com/Iron-E/clinvoice/issues"),
	})
}
