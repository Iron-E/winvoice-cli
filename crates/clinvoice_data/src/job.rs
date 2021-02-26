mod from_view;
mod hash;

use
{
	crate::{Decimal, Expense, Id, Invoice, Money, Timesheet},
	chrono::{DateTime, Utc},
};

#[cfg(feature="serde_support")]
use serde::{Deserialize, Serialize};

const MINUTES_PER_HOUR: i8 = 60;

/// # Summary
///
/// A [`Job`] contains all of the information which pertains to the specific
/// reasons that a [client](Organization) has contacted the user's
/// [employer](Organization) / the user.
///
/// It also defines the scope of the problem which is to be solved before an
/// [`Invoice`] is issued.
///
/// # Remarks
///
/// The [`Job`] can be thought of similarly to a support ticket. Whereas other
/// structures may define [the method of payment](Invoice),
/// [client](Organization) information, and [work periods](Timesheet)— this
/// structure defines what work _may_ performed.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde_support", derive(Deserialize, Serialize))]
pub struct Job
{
	/// # Summary
	///
	/// The [`Organization`](crate::Organization) who the work is being performed for.
	pub client_id: Id,

	/// # Summary
	///
	/// The date upon which the client accepted the work as "complete".
	pub date_close: Option<DateTime<Utc>>,

	/// # Summary
	///
	/// The [date](DateTime) upon which the client requested the work.
	pub date_open: DateTime<Utc>,

	/// # Summary
	///
	/// The __unique__ number of the [`Job`].
	///
	/// # Remarks
	///
	/// Should be automatically generated.
	pub id: Id,

	/// # Summary
	///
	/// The [`Invoice`] which will be sent to the [client](Organization) after the [`Job`] is done.
	pub invoice: Invoice,

	/// # Summary
	///
	/// Important things to know about the work that has been performed.
	///
	/// # Example
	///
	/// > __Note:__ the `str` may contain any valid markdown.
	///
	/// ```markdown
	/// * Images on the website now point to the correct location.
	/// * The PDF application has been replaced with a Google Form.
	/// * Customer support has been contacted and will reach out to you within X days.
	/// ```
	pub notes: String,

	/// # Summary
	///
	/// What problems will be addressed before the [`Job`] is closed.
	///
	/// # Example
	///
	/// > __Note:__ the `str` may contain any valid markdown.
	///
	/// ```markdown
	/// * Fix website rendering issue.
	/// * Replace PDF with Google Form.
	/// * Contact customer support for X hardware device.
	/// ```
	pub objectives: String,

	/// # Summary
	///
	/// The periods of time during which work was performed for this [`Job`].
	pub timesheets: Vec<Timesheet>,
}

impl Job
{
	/// # Summary
	///
	/// Create a new [`Timesheet`] and attach it to the list of [`timesheets`](Self::timesheets).
	///
	/// # Remarks
	///
	/// * This is intended to be used for reporting work which was done previously.
	pub fn attach_timesheet(&mut self, employee: Id, expenses: Option<Vec<Expense>>, time_begin: DateTime<Utc>, time_end: Option<DateTime<Utc>>, work_notes: &str)
	{
		self.timesheets.push(
			Timesheet
			{
				employee_id: employee,
				expenses,
				time_begin,
				time_end,
				work_notes: work_notes.into()
			}
		);
	}

	/// # Summary
	///
	/// Create a new [`Timesheet`] with the starting time set to the current time.
	///
	/// # Remarks
	///
	/// * This is a synonym for [`attach_timesheet`](Self::attach_timesheet) but with the `time_start`
	///   parameter defaulted to [`Utc::now`](chrono::Utc::now).
	/// * This is intended to be used for reporting work which is about to be done.
	///
	/// # Parameters
	///
	/// * `employee`, the [`Id`] of the [`Employee`] who is working on this timesheet.
	pub fn start_timesheet(&mut self, employee: Id)
	{
		self.attach_timesheet(employee, None, Utc::now(), None, "* Work which was done goes here.\n* Supports markdown formatting.");
	}

	/// # Summary
	///
	/// Get the amount of [`Money`] which is owed by the client on the [`Inovice`].
	///
	/// # Panics
	///
	/// * When not all [`Money`] amounts are in the same currency.
	///     * TODO: add currency conversion method.
	pub fn total(&self) -> Money
	{
		let minutes_per_hour = Decimal::from(MINUTES_PER_HOUR);
		let seconds_per_minute = minutes_per_hour;

		let mut total = Money
		{
			amount: Decimal::new(0, 2),
			currency: self.invoice.hourly_rate.currency.clone(),
		};

		for timesheet in self.timesheets.iter().filter(|t| t.time_end.is_some())
		{
			let duration_seconds = Decimal::from(timesheet.time_end.unwrap().signed_duration_since(timesheet.time_begin).num_seconds());
			total.amount += (duration_seconds / seconds_per_minute / minutes_per_hour) * self.invoice.hourly_rate.amount;

			if let Some(expenses) = &timesheet.expenses
			{
				for expense in expenses
				{
					if expense.cost.currency == total.currency
					{
						panic!("Not all expenses were recorded in the same currency! There is currently no automatic currency conversion.");
					}

					total.amount += expense.cost.amount;
				}
			}
		}

		total
	}
}
