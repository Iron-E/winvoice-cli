mod from_view;

use chrono::{DateTime, Utc};
use clinvoice_finance::{Decimal, ExchangeRates, Money, Result};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::{Expense, Id};

const SECONDS_PER_HOUR: i16 = 3600;

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
	/// Get the amount of [`Money`] which is owed by the client on the [`Inovice`].
	///
	/// # Panics
	///
	/// * When not all [`Money`] amounts are in the same currency.
	pub fn total(hourly_rate: Money, timesheets: &[Timesheet]) -> Result<Money>
	{
		let seconds_per_hour: Decimal = SECONDS_PER_HOUR.into();

		let mut exchange_rates = None;

		let mut total = timesheets
			.iter()
			.filter(|timesheet| timesheet.time_end.is_some())
			.try_fold(
				Money::new(0, 2, hourly_rate.currency),
				|mut total, timesheet| -> Result<Money> {
					let duration_seconds: Decimal = timesheet
						.time_end
						.expect(
							"Previous iterator filter should have assured end time of Timesheet had a \
							 value",
						)
						.signed_duration_since(timesheet.time_begin)
						.num_seconds()
						.into();
					total.amount += (duration_seconds / seconds_per_hour) * hourly_rate.amount;

					timesheet
						.expenses
						.iter()
						.try_for_each(|expense| -> Result<()> {
							if expense.cost.currency == total.currency
							{
								total.amount += expense.cost.amount;
							}
							else
							{
								if exchange_rates.is_none()
								{
									exchange_rates = Some(ExchangeRates::new()?);
								}

								total.amount += expense
									.cost
									.exchange(
										total.currency,
										exchange_rates.as_ref().expect(
											"The exchange rates should have been downloaded just before this",
										),
									)
									.amount;
							}

							Ok(())
						})?;

					Ok(total)
				},
			)?;

		total.amount.rescale(2);
		Ok(total)
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
			Timesheet::total(Money::new(20_00, 2, Currency::USD), &timesheets).unwrap(),
			Money::new(4000, 2, Currency::USD),
		);
	}
}
