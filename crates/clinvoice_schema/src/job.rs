mod default;
mod display;
mod restorable_serde;

use core::{fmt::Write, time::Duration};
use std::collections::HashSet;

use chrono::{DateTime, Local, Utc};
use clinvoice_finance::{ExchangeRates, Result};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{
	markdown::{Element, Text},
	Id,
	Invoice,
	Organization,
	Timesheet,
};

/// # Summary
///
/// A view of [`Job`](crate::Job).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Job
{
	/// # Summary
	///
	/// The [`Organization`](crate::Organization) who the work is being performed for.
	pub client: Organization,

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
	/// The [`Job`] number.
	#[cfg_attr(feature = "serde_support", serde(skip))]
	pub id: Id,

	/// # Summary
	///
	/// The amount of time between increments to the [`time_end`] on a [`Timesheet`].
	///
	/// # Example
	///
	/// * If `increment` is 15m…
	///   * A work begin time of 12:14 is set to 12:15.
	///   * A work end time of 13:29 is set to 13:30.
	/// * If `increment` is 5m…
	///   * A work begin time of 12:07 is set to 12:05.
	///   * A work end time of 13:31 is set to 13:30.
	/// * If `increment` is 0m…
	///   * A work begin time of 12:14 is not changed.
	///   * A work end time of 13:29 is not changed.
	///
	/// __Note__ that the duration does not have to be in even minutes. It can be any combination of
	/// days, hours, minutes, etc.
	#[serde(with = "humantime_serde")]
	pub increment: Duration,

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
}

impl Job
{
	/// # Summary
	///
	/// Export some `job` to the [`Target`] specified.
	pub fn export(
		&self,
		exchange_rates: Option<&ExchangeRates>,
		timesheets: &[Timesheet],
	) -> Result<String>
	{
		let mut output = String::new();

		writeln!(output, "{}", Element::Heading {
			depth: 1,
			text: format!("Job #{}", self.id),
		})
		.unwrap();

		writeln!(
			output,
			"{}: {}",
			Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Client"),
			},
			self.client,
		)
		.unwrap();

		writeln!(
			output,
			"{}: {}",
			Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Date Opened"),
			},
			DateTime::<Local>::from(self.date_open),
		)
		.unwrap();

		if let Some(date) = self.date_close
		{
			writeln!(
				output,
				"{}: {}",
				Element::UnorderedList {
					depth: 0,
					text: Text::Bold("Date Closed"),
				},
				DateTime::<Local>::from(date).naive_local(),
			)
			.unwrap();
		}

		writeln!(output, "{}", Element::<&str>::Break).unwrap();

		writeln!(output, "{}", Element::Heading {
			depth: 2,
			text: "Invoice",
		})
		.unwrap();
		writeln!(
			output,
			"{} {}",
			Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Hourly Rate"),
			},
			self.invoice.hourly_rate,
		)
		.unwrap();

		if let Some(date) = &self.invoice.date
		{
			writeln!(
				output,
				"{}: {}",
				Element::UnorderedList {
					depth: 0,
					text: Text::Bold("Status"),
				},
				date,
			)
			.unwrap();
		}

		writeln!(
			output,
			"{}: {}",
			Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Total Amount Owed"),
			},
			Timesheet::total(exchange_rates, self.invoice.hourly_rate, timesheets),
		)
		.unwrap();
		writeln!(output, "{}", Element::<&str>::Break).unwrap();

		writeln!(output, "{}", Element::Heading {
			depth: 2,
			text: "Objectives",
		})
		.unwrap();
		writeln!(output, "{}", Element::BlockText(&self.objectives)).unwrap();

		if !self.notes.is_empty()
		{
			writeln!(output, "{}", Element::Heading {
				depth: 2,
				text: "Notes",
			})
			.unwrap();
			writeln!(output, "{}", Element::BlockText(&self.notes)).unwrap();
		}

		if !timesheets.is_empty()
		{
			writeln!(output, "{}", Element::Heading {
				depth: 2,
				text: "Timesheets",
			})
			.unwrap();
			let mut employees = HashSet::new();
			timesheets
				.iter()
				.for_each(|t| t.export(&mut employees, &mut output));
		}

		Ok(output)
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;
	use std::collections::HashMap;

	use chrono::Utc;
	use clinvoice_finance::{Currency, Money};

	use super::{DateTime, Job, Local, Timesheet};
	use crate::{Employee, Expense, Invoice, Location, Organization, Person};

	#[test]
	fn export()
	{
		let organization = Organization {
			id: 0,
			location: Location {
				id: 0,
				outer: Some(
					Location {
						id: 0,
						outer: Some(
							Location {
								id: 0,
								outer: Some(
									Location {
										id: 0,
										outer: Some(
											Location {
												id: 0,
												outer: None,
												name: "Earth".into(),
											}
											.into(),
										),
										name: "USA".into(),
									}
									.into(),
								),
								name: "Arizona".into(),
							}
							.into(),
						),
						name: "Phoenix".into(),
					}
					.into(),
				),
				name: "1337 Some Street".into(),
			},
			name: "Big Old Test".into(),
		};

		let testy_mctesterson = Employee {
			contact_info: vec![].into_iter().collect(),
			id: 0,
			organization: organization.clone(),
			person: Person {
				id: 0,
				name: "Testy McTesterson".into(),
			},
			status: "Representative".into(),
			title: "CEO of Tests".into(),
		};

		let bob = Employee {
			contact_info: HashMap::new(),
			id: 0,
			organization: organization.clone(),
			person: Person {
				id: 0,
				name: "Bob".into(),
			},
			status: "Employed".into(),
			title: "Janitor".into(),
		};

		let mut job = Job {
			client: organization,
			date_close: None,
			date_open: Utc::today().and_hms(0, 0, 0),
			id: 0,
			increment: Duration::from_secs(900),
			invoice: Invoice {
				date: None,
				hourly_rate: Money::new(20_00, 2, Currency::USD),
			},
			notes: "- I tested the function.".into(),
			objectives: "- I want to test this function.".into(),
		};

		assert_eq!(
			job.export(None, &[]).unwrap(),
			format!(
				"# Job #{}

- **Client**: Big Old Test @ 1337 Some Street, Phoenix, Arizona, USA, Earth
- **Date Opened**: {}

## Invoice

- **Hourly Rate** 20.00 USD
- **Total Amount Owed**: 0.00 USD

## Objectives

- I want to test this function.

## Notes

- I tested the function.\n\n",
				job.id,
				DateTime::<Local>::from(job.date_open),
			),
		);

		job.date_close = Some(Utc::today().and_hms(4, 30, 0));

		let timesheets = vec![
			Timesheet {
				employee: testy_mctesterson,
				job: job.clone(),
				time_begin: Utc::today().and_hms(2, 0, 0),
				time_end: Some(Utc::today().and_hms(2, 30, 0)),
				work_notes: "- Wrote the test.".into(),
				..Default::default()
			},
			Timesheet {
				employee: bob,
				expenses: vec![Expense {
					id: 120,
					category: "Service".into(),
					cost: Money::new(20_00, 2, Currency::USD),
					description: "Paid for someone else to clean".into(),
				}],
				job: job.clone(),
				time_begin: Utc::today().and_hms(3, 0, 0),
				time_end: Some(Utc::today().and_hms(3, 30, 0)),
				work_notes: "- Clean the deck.".into(),
				..Default::default()
			},
		];

		assert_eq!(
			job.export(None, &timesheets).unwrap(),
			format!(
				"# Job #{}

- **Client**: Big Old Test @ 1337 Some Street, Phoenix, Arizona, USA, Earth
- **Date Opened**: {}
- **Date Closed**: {}

## Invoice

- **Hourly Rate** 20.00 USD
- **Total Amount Owed**: 40.00 USD

## Objectives

- I want to test this function.

## Notes

- I tested the function.

## Timesheets

### {} – {}

#### Employee Information

- **Name**: Testy McTesterson
- **Employer**: Big Old Test @ 1337 Some Street, Phoenix, Arizona, USA, Earth
- **Title**: CEO of Tests

#### Work Notes

- Wrote the test.

### {} – {}

#### Employee Information

- **Name**: Bob
- **Employer**: Big Old Test @ 1337 Some Street, Phoenix, Arizona, USA, Earth
- **Title**: Janitor

#### Expenses

##### #120 – Service (20.00 USD)

Paid for someone else to clean

#### Work Notes

- Clean the deck.\n\n",
				job.id,
				DateTime::<Local>::from(job.date_open),
				DateTime::<Local>::from(job.date_close.unwrap()).naive_local(),
				timesheets[0].time_begin,
				timesheets[0].time_end.unwrap().naive_local(),
				timesheets[1].time_begin,
				timesheets[1].time_end.unwrap().naive_local(),
			),
		);
	}
}
