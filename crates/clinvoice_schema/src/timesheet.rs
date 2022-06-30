mod default;
mod display;
mod exchangeable;
mod restorable_serde;

use chrono::{DateTime, Utc};
use clinvoice_finance::{Decimal, ExchangeRates, Exchangeable, Money};
use lazy_static::lazy_static;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{Employee, Expense, Id, Job};

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
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Timesheet
{
	/// # Summary
	///
	/// The ID of this [`Timesheet`](crate::Employee).
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// # Summary
	///
	/// The ID of the [`Employee`](crate::Employee) who performed this work.
	pub employee: Employee,

	/// # Summary
	///
	/// [`Expense`]s which were incurred during this time.
	pub expenses: Vec<Expense>,

	/// # Summary
	///
	/// The ID of the [`Job`](crate::Job) this [`Timesheet`] is attached to.
	pub job: Job,

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
		timesheets: &[Self],
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
						.expect("Filters should have assured that `Timesheet`s have an end time")
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
			id: 0,
			time_begin: Utc::today().and_hms(2, 0, 0),
			time_end: Some(Utc::today().and_hms(2, 30, 0)),
			work_notes: "- Wrote the test.".into(),
			..Default::default()
		});

		timesheets.push(Timesheet {
			expenses: vec![Expense {
				id: 102,
				category: "Item".into(),
				cost: Money::new(20_00, 2, Currency::Usd),
				description: "Paid for someone else to clean".into(),
				..Default::default()
			}],
			time_begin: Utc::today().and_hms(3, 0, 0),
			time_end: Some(Utc::today().and_hms(3, 30, 0)),
			work_notes: "- Clean the deck.".into(),
			..Default::default()
		});

		assert_eq!(
			Timesheet::total(None, Money::new(20_00, 2, Currency::Usd), &timesheets),
			Money::new(4000, 2, Currency::Usd),
		);
	}
}
