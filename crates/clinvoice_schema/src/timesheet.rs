mod default;
mod display;
mod restorable_serde;

use core::fmt::Write;
use std::collections::HashSet;

use chrono::{DateTime, Utc};
use clinvoice_finance::{Decimal, ExchangeRates, Money};
use lazy_static::lazy_static;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{
	markdown::{Element, Text},
	Employee,
	Job,
};
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
	/// Export some `job` to the [`Target`] specified. Appends to some pre-existing `output`, in
	/// case multiple [`Timesheet`]s must be serialized sequentially.
	///
	/// Tracks the `organizations_with_serialized_contact_info` so that their contact information is not
	/// reiterated every time.
	pub(super) fn export(
		&self,
		organizations_with_serialized_contact_info: &mut HashSet<Id>,
		output: &mut String,
	)
	{
		writeln!(output, "{}", Element::Heading {
			depth: 3,
			text: self
				.time_end
				.map(|time_end| format!("{} – {}", self.time_begin, time_end.naive_local()))
				.unwrap_or_else(|| format!("{} – Current", self.time_begin)),
		})
		.unwrap();

		writeln!(
			output,
			"{}: {} {}",
			Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Employee"),
			},
			self.employee.title,
			self.employee.name,
		)
		.unwrap();

		writeln!(
			output,
			"{}: {}",
			Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Employer"),
			},
			self.employee.organization,
		)
		.unwrap();

		if !organizations_with_serialized_contact_info.contains(&self.employee.id)
		{
			let employee_contact_info: Vec<_> = self
				.employee
				.organization
				.contact_info
				.iter()
				.filter(|c| c.export)
				.collect();

			if !employee_contact_info.is_empty()
			{
				writeln!(output, "{}:", Element::UnorderedList {
					depth: 0,
					text: Text::Bold("Contact Information"),
				})
				.unwrap();

				let mut sorted_organization_contact_info = employee_contact_info;
				sorted_organization_contact_info.sort_by_key(|c| &c.label);

				sorted_organization_contact_info
					.into_iter()
					.try_for_each(|contact| {
						writeln!(
							output,
							"{}: {}",
							Element::UnorderedList {
								depth: 1,
								text: Text::Bold(&contact.label),
							},
							// The part we want is in `[`, `]`.
							// The matches are in `(`, `)`.
							// "Multiple colons(: )this is the end(: )[555-555-5555]"
							contact.to_string().split(": ").last().unwrap_or_default(),
						)
					})
					.unwrap();
			}
		}

		writeln!(output, "{}", Element::<&str>::Break).unwrap();

		if !self.expenses.is_empty()
		{
			writeln!(output, "{}", Element::Heading {
				depth: 4,
				text: "Expenses",
			})
			.unwrap();

			self
				.expenses
				.iter()
				.try_for_each(|e| {
					writeln!(
						output,
						"{}\n{}",
						Element::Heading {
							depth: 5,
							text: format!("#{} – {} ({})", e.id, e.category, e.cost),
						},
						Element::BlockText(&e.description),
					)
				})
				.unwrap();
		}

		if !self.work_notes.is_empty()
		{
			writeln!(output, "{}", Element::Heading {
				depth: 4,
				text: "Work Notes",
			})
			.unwrap();
			writeln!(output, "{}", Element::BlockText(&self.work_notes)).unwrap();
		}

		organizations_with_serialized_contact_info.insert(self.employee.organization.id);
	}

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
				cost: Money::new(20_00, 2, Currency::USD),
				description: "Paid for someone else to clean".into(),
				..Default::default()
			}],
			time_begin: Utc::today().and_hms(3, 0, 0),
			time_end: Some(Utc::today().and_hms(3, 30, 0)),
			work_notes: "- Clean the deck.".into(),
			..Default::default()
		});

		assert_eq!(
			Timesheet::total(None, Money::new(20_00, 2, Currency::USD), &timesheets),
			Money::new(4000, 2, Currency::USD),
		);
	}
}
