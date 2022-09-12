mod default;
mod display;
mod from_str;

use clinvoice_schema::chrono::{DateTime, NaiveDateTime, Utc};

use crate::utils;

/// Can be used with [`clap`] to create options with optional arguments e.g. `--foo` and `--foo
/// bar`.
///
/// ```sh
/// clinvoice           # FlagOrArgument::Flag(false)
/// clinvoice --foo     # FlagOrArgument::Flag(true)
/// clinvoice --foo bar # FlagOrArgument::Argument("bar")
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FlagOrArgument<T>
{
	/// A option with an argument e.g. "bar" for `clinvoice --foo bar`.
	Argument(T),

	/// A flag with no argument e.g. `clinvoice`, `clinvoice --foo`.
	Flag(bool),
}

impl<T> FlagOrArgument<T>
{
	/// Returns [`Some`] if an [`Argument`](FlagOrArgument::Argument) was provided, or [`None`] if
	/// a [`Flag`](FlagOrArgument::Flag) was provided.
	#[allow(clippy::missing_const_for_fn)]
	pub fn argument(self) -> Option<T>
	{
		match self
		{
			Self::Argument(a) => Some(a),
			Self::Flag(_) => None,
		}
	}

	/// Returns the [`Flag`](FlagOrArgument::Flag) value, or `true` if an
	/// [`Argument`](FlagOrArgument::Argument) was provided.
	pub const fn flag(&self) -> bool
	{
		match self
		{
			Self::Argument(_) => true,
			Self::Flag(b) => *b,
		}
	}
}

impl FlagOrArgument<NaiveDateTime>
{
	/// IFF [`flag`][flag]ged, map [`Self::Argument`] to its [`DateTime<Utc>`] or default to
	/// [`Utc::now`].
	///
	/// IFF not [`flag`][flag]ged, return [`None`].
	///
	/// [flag]: Self::flag
	pub fn iff_flagged_utc_or_now(self) -> Option<DateTime<Utc>>
	{
		self.flag()
			.then(|| self.argument().map_or_else(Utc::now, utils::naive_local_datetime_to_utc))
	}
}
