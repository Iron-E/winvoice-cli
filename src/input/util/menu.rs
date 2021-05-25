use crate::input;

pub const ADD: &str = "Add";
pub const CONTINUE: &str = "Continue";
pub const DELETE: &str = "Delete";
pub const EDIT: &str = "Edit";
pub const ALL_ACTIONS: [&str; 4] = [ADD, CONTINUE, DELETE, EDIT];

pub const NO: &str = "No";
pub const YES: &str = "Yes";

pub fn confirm(prompt: impl Into<String>) -> input::Result<bool>
{
	const OPTIONS: [&str; 2] = [YES, NO];
	input::select_one(&OPTIONS, prompt).map(|confirmed| match confirmed
	{
		YES => true,
		NO => false,
		_ => unreachable!("Unrecognized confirmation. Please file an issue at https://github.com/Iron-E/clinvoice/issues"),
	})
}
