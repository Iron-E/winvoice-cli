#![allow(clippy::use_self)]

use strum::{Display, EnumIter};

/// A possible action to choose while using the [`menu`](super::menu).
#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Action
{
	/// Queue another [`Expense`] to be created.
	Add,

	/// Exit the menu, saving the changes.
	Continue,

	/// Deque an [`Expense`] from being created.
	Delete,

	/// Edit a [`Expense`] that was queued to be created.
	Edit,
}
