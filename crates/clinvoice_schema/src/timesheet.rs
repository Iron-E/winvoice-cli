mod from_view;

use chrono::{DateTime, Utc};
use clinvoice_finance::{Decimal, ExchangeRates, Money};
use lazy_static::lazy_static;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::{Expense, Id};

/// # Summary
///
/// A `Timesheet` contains all information pertaining to work that has been performed during a
/// specific period of time while working on a [`Job`](super::job::Job)
///
/// # Remarks
///
/// It is likely that a given CLInvoice business object will contain multiple timesheets. As such,
/// it is proposed that the container for business logic contain an array of `Timesheet`, rather
/// than only one.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Timesheet
{
	/// # Summary
	///
	/// The ID of the [`Employee`](crate::Employee) who performed this work.
	pub employee_id: Id,

	/// # Summary
	///
	/// [`Expense`]s which were incurred during this time.
	pub expenses: Vec<Expense>,

	/// # Summary
	///
	/// The ID of the [`Job`](crate::Job) this [`Timesheet`] is attached to.
	pub job_id: Id,

	/// # Summary
	///
	/// The time at which this period of work began.
	pub time_begin: DateTime<Utc>,

	/// # Summary
	///
	/// The time at which this period of work ended.
	///
	/// # Remarks
	///
	/// Is [`Option`] because the time that a work period ends is not known upon first creation.
	pub time_end: Option<DateTime<Utc>>,

	/// # Summary
	///
	/// A summary of what work was performed
	///
	/// # Example
	///
	/// > __Note:__ the `str` may contain any valid markdown.
	///
	/// ```markdown
	/// * Researched alternative solutions to image rendering issue.
	/// * Implemented chosen solution.
	/// * Created tests for chosen solution.
	/// ```
	pub work_notes: String,
}

impl Timesheet
{
	/// # Summary
	///
	/// Get the amount of [`Money`] which is owed by the client on the [`Inovice`](crate::Invoice).
	///
	/// # Panics
	///
	/// When currencies must be exchanged, but the `exchange_rates` are not provided.
	///
	/// * This usually happens when `hourly_rate` is not the [`ExchangeRates::default`], as that is
	///   the currency which is used for storage.
	pub fn total(
		exchange_rates: Option<&ExchangeRates>,
		hourly_rate: Money,
		timesheets: &[Timesheet],
	) -> Money
	{
		lazy_static! {
			static ref SECONDS_PER_HOUR: Decimal = 3600.into();
		}

		let mut total = Money::new(0, 2, hourly_rate.currency);
		timesheets
			.iter()
			.filter(|timesheet| timesheet.time_end.is_some())
			.for_each(|timesheet| {
				total.amount += (Decimal::from(
					timesheet
						.time_end
						.expect("Filters should assure that `Timesheet`s have an end time")
						.signed_duration_since(timesheet.time_begin)
						.num_seconds(),
				) / *SECONDS_PER_HOUR) *
					hourly_rate.amount;

				timesheet.expenses.iter().for_each(|expense| {
					total.amount += if expense.cost.currency == total.currency
					{
						expense.cost.amount
					}
					else
					{
						expense
							.cost
							.exchange(
								total.currency,
								exchange_rates.unwrap_or_else(|| {
									panic!(
										"Must do currency conversion from {} to {}, but the exchange rates \
										 were not provided.",
										expense.cost.currency, total.currency
									)
								}),
							)
							.amount
					}
				});
			});

		total.amount.rescale(2);
		total
	}
}

#[cfg(test)]
mod tests
{
	use chrono::Utc;
	use clinvoice_finance::Currency;

	use super::{Expense, Money, Timesheet};

	#[test]
	fn total()
	{
		let mut timesheets = Vec::new();

		timesheets.push(Timesheet {
			employee_id: 0,
			expenses: Vec::new(),
			job_id: 0,
			time_begin: Utc::today().and_hms(2, 0, 0),
			time_end: Some(Utc::today().and_hms(2, 30, 0)),
			work_notes: "- Wrote the test.".into(),
		});

		timesheets.push(Timesheet {
			employee_id: 0,
			expenses: vec![Expense {
				category: "Item".into(),
				cost: Money::new(20_00, 2, Currency::USD),
				description: "Paid for someone else to clean".into(),
			}],
			job_id: 0,
			time_begin: Utc::today().and_hms(3, 0, 0),
			time_end: Some(Utc::today().and_hms(3, 30, 0)),
			work_notes: "- Clean the deck.".into(),
		});

		assert_eq!(
			Timesheet::total(None, Money::new(20_00, 2, Currency::USD), &timesheets),
			Money::new(4000, 2, Currency::USD),
		);
	}
}
