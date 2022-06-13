mod default;
mod display;
mod exchangeable;
mod restorable_serde;

use core::{fmt::Write, time::Duration};

use chrono::{DateTime, Local, Utc};
use clinvoice_finance::ExchangeRates;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::{
	markdown::{Element, Text},
	Id,
	Invoice,
	Organization,
	Timesheet,
};
use crate::Contact;

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
	#[cfg_attr(feature = "serde_support", serde(with = "humantime_serde"))]
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
		contact_info: &mut [Contact],
		exchange_rates: Option<&ExchangeRates>,
		organization: &Organization,
		timesheets: &[Timesheet],
	) -> String
	{
		let mut output = String::new();

		writeln!(output, "{}", Element::Heading {
			depth: 1,
			text: format!("{} – Job №{}", organization.name, self.id),
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
				DateTime::<Local>::from(date),
			)
			.unwrap();
		}

		writeln!(output, "{}", Element::<&str>::Break).unwrap();

		if !self.objectives.is_empty()
		{
			writeln!(output, "{}", Element::Heading {
				depth: 2,
				text: "Objectives",
			})
			.unwrap();

			writeln!(output, "{}", Element::BlockText(&self.objectives)).unwrap();
		}

		if !self.notes.is_empty()
		{
			writeln!(output, "{}", Element::Heading {
				depth: 2,
				text: "Notes",
			})
			.unwrap();

			writeln!(output, "{}", Element::BlockText(&self.notes)).unwrap();
		}

		writeln!(output, "{}", Element::Heading {
			depth: 2,
			text: "Invoice",
		})
		.unwrap();

		if !contact_info.is_empty()
		{
			writeln!(output, "{}:", Element::UnorderedList {
				depth: 0,
				text: Text::Bold("Contact Information"),
			})
			.unwrap();

			let sorted_contact_info = contact_info;
			sorted_contact_info.sort_by(|c1, c2| c1.label.cmp(&c2.label));

			sorted_contact_info
				.iter()
				.try_for_each(|contact| {
					writeln!(
						output,
						"{}: {}",
						Element::UnorderedList {
							depth: 1,
							text: Text::Bold(&contact.label),
						},
						contact.kind,
					)
				})
				.unwrap();
		}

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

		if !timesheets.is_empty()
		{
			writeln!(output, "{}", Element::Heading {
				depth: 2,
				text: "Timesheets",
			})
			.unwrap();

			timesheets
				.iter()
				.for_each(|timesheet| timesheet.export(&mut output));
		}

		output
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;

	use chrono::Utc;
	use clinvoice_finance::{Currency, Money};

	use super::{DateTime, Job, Local, Timesheet};
	use crate::{Contact, ContactKind, Employee, Expense, Invoice, Location, Organization};

	#[test]
	fn export()
	{
		let client = Organization {
			id: 0,
			location: Location {
				id: 0,
				outer: Some(
					Location {
						id: 1,
						outer: Some(
							Location {
								id: 2,
								outer: Some(
									Location {
										id: 3,
										outer: Some(
											Location {
												id: 4,
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

		let testy_organization = Organization {
			id: 1,
			name: "TestyCo".into(),
			location: client.location.clone(),
		};

		let mut contact_info = [
			Contact {
				kind: ContactKind::Email("foo@bar.io".into()),
				label: "primary email".into(),
			},
			Contact {
				kind: ContactKind::Phone("687 5309".into()),
				label: "primary phone".into(),
			},
			Contact {
				kind: ContactKind::Username("TestyCo".into()),
				label: "twitter".into(),
			},
		];

		let testy_mctesterson = Employee {
			id: Default::default(),
			name: "Testy McTesterson".into(),
			status: "Representative".into(),
			title: "CEO of Tests".into(),
		};

		let bob = Employee {
			id: Default::default(),
			name: "Bob".into(),
			status: "Employed".into(),
			title: "Janitor".into(),
		};

		let mut job = Job {
			client,
			date_close: None,
			date_open: Utc::today().and_hms(0, 0, 0),
			id: Default::default(),
			increment: Duration::from_secs(900),
			invoice: Invoice {
				date: None,
				hourly_rate: Money::new(20_00, 2, Currency::USD),
			},
			notes: "- I tested the function.".into(),
			objectives: "- I want to test this function.".into(),
		};

		assert_eq!(
			job.export(&mut contact_info, None, &testy_organization, &[]),
			format!(
				"# TestyCo – Job №{}

- **Client**: Big Old Test @ 1337 Some Street, Phoenix, Arizona, USA, Earth
- **Date Opened**: {}

## Objectives

- I want to test this function.

## Notes

- I tested the function.

## Invoice

- **Contact Information**:
	- **primary email**: foo@bar.io
	- **primary phone**: 687 5309
	- **twitter**: TestyCo
- **Hourly Rate** 20.00 USD
- **Total Amount Owed**: 0.00 USD\n\n",
				job.id,
				DateTime::<Local>::from(job.date_open),
			),
		);

		job.date_close = Some(Utc::today().and_hms(4, 30, 0));

		let timesheets = [
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
					..Default::default()
				}],
				job: job.clone(),
				time_begin: Utc::today().and_hms(3, 0, 0),
				time_end: Some(Utc::today().and_hms(3, 30, 0)),
				work_notes: "- Clean the deck.".into(),
				..Default::default()
			},
		];

		assert_eq!(
			job.export(&mut contact_info, None, &testy_organization, &timesheets),
			format!(
				"# TestyCo – Job №{}

- **Client**: Big Old Test @ 1337 Some Street, Phoenix, Arizona, USA, Earth
- **Date Opened**: {}
- **Date Closed**: {}

## Objectives

- I want to test this function.

## Notes

- I tested the function.

## Invoice

- **Contact Information**:
	- **primary email**: foo@bar.io
	- **primary phone**: 687 5309
	- **twitter**: TestyCo
- **Hourly Rate** 20.00 USD
- **Total Amount Owed**: 40.00 USD

## Timesheets

### {} – {}

- **Employee**: CEO of Tests Testy McTesterson

#### Work Notes

- Wrote the test.

### {} – {}

- **Employee**: Janitor Bob

#### Expenses

##### №{} – Service (20.00 USD)

Paid for someone else to clean

#### Work Notes

- Clean the deck.\n\n",
				job.id,
				DateTime::<Local>::from(job.date_open),
				DateTime::<Local>::from(job.date_close.unwrap()),
				timesheets[0].time_begin,
				timesheets[0].time_end.unwrap(),
				timesheets[1].time_begin,
				timesheets[1].time_end.unwrap(),
				timesheets[1].expenses[0].id,
			),
		);
	}
}
