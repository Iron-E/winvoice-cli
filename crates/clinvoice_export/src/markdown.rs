//! This module contains types which [`Display`] as valid Markdown syntax. It provides a
//! [function](export_job) to export a [`Job`](clinvoice_schema::Job) as
//! [Markdown](crate::Format::Markdown).

mod block;
mod text;

use core::fmt::Write;

pub use block::Block;
use clinvoice_finance::ExchangeRates;
use clinvoice_schema::{
	chrono::{DateTime, Local},
	Contact,
	Job,
	Organization,
	Timesheet,
};
pub use text::Text;

/// Export some `job` to [Markdown](crate::Format::Markdown).
///
/// `contact_info` and `timesheets` are exported in the order given.
///
/// # Warnings
///
/// * The following fields must all contain valid markdown syntax:
///   * The `objectives` and `notes` of the `job`.
///   * The `work_notes` of every [`Timesheet`] of the `timesheets`.
///   * The `category` and `description` of every [`Expense`] of the `expenses` of every [`Timesheet`] of
///     the `timesheets`.
///
/// # Panics
///
/// * When [`Timesheet::total`](clinvoice_schema::Timesheet::total) does.
pub(super) fn export_job(
	job: &Job,
	contact_info: &[Contact],
	exchange_rates: Option<&ExchangeRates>,
	organization: &Organization,
	timesheets: &[Timesheet],
) -> String
{
	let mut output = String::new();

	writeln!(output, "{}", Block::Heading {
		indents: 1,
		text: format!("{} – Job №{}", organization.name, job.id),
	})
	.unwrap();

	writeln!(
		output,
		"{}: {}",
		Block::UnorderedList {
			indents: 0,
			text: Text::Bold("Client"),
		},
		job.client,
	)
	.unwrap();

	writeln!(
		output,
		"{}: {}",
		Block::UnorderedList {
			indents: 0,
			text: Text::Bold("Date Opened"),
		},
		DateTime::<Local>::from(job.date_open),
	)
	.unwrap();

	if let Some(date) = job.date_close
	{
		writeln!(
			output,
			"{}: {}",
			Block::UnorderedList {
				indents: 0,
				text: Text::Bold("Date Closed"),
			},
			DateTime::<Local>::from(date),
		)
		.unwrap();
	}

	writeln!(output, "{}", Block::<&str>::Break).unwrap();

	if !job.objectives.is_empty()
	{
		writeln!(output, "{}", Block::Heading {
			indents: 2,
			text: "Objectives",
		})
		.unwrap();

		writeln!(output, "{}", Block::Text(&job.objectives)).unwrap();
	}

	if !job.notes.is_empty()
	{
		writeln!(output, "{}", Block::Heading {
			indents: 2,
			text: "Notes",
		})
		.unwrap();

		writeln!(output, "{}", Block::Text(&job.notes)).unwrap();
	}

	writeln!(output, "{}", Block::Heading {
		indents: 2,
		text: "Invoice",
	})
	.unwrap();

	if !contact_info.is_empty()
	{
		writeln!(output, "{}:", Block::UnorderedList {
			indents: 0,
			text: Text::Bold("Contact Information"),
		})
		.unwrap();

		contact_info
			.iter()
			.try_for_each(|contact| {
				writeln!(
					output,
					"{}: {}",
					Block::UnorderedList {
						indents: 1,
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
		Block::UnorderedList {
			indents: 0,
			text: Text::Bold("Hourly Rate"),
		},
		job.invoice.hourly_rate,
	)
	.unwrap();

	if let Some(date) = &job.invoice.date
	{
		writeln!(
			output,
			"{}: {}",
			Block::UnorderedList {
				indents: 0,
				text: Text::Bold("Status"),
			},
			date,
		)
		.unwrap();
	}

	writeln!(
		output,
		"{}: {}",
		Block::UnorderedList {
			indents: 0,
			text: Text::Bold("Total Amount Owed"),
		},
		Timesheet::total_all(timesheets, exchange_rates, job.invoice.hourly_rate),
	)
	.unwrap();

	writeln!(output, "{}", Block::<&str>::Break).unwrap();

	if !timesheets.is_empty()
	{
		writeln!(output, "{}", Block::Heading {
			indents: 2,
			text: "Timesheets",
		})
		.unwrap();

		timesheets
			.iter()
			.for_each(|timesheet| export_timesheet(timesheet, &mut output));
	}

	output
}

/// Export some `timesheet` to [Markdown](crate::Format::Markdown).
///
/// # Warnings
///
/// * The following fields must all contain valid markdown syntax:
///   * The `work_notes` of the `timesheet`.
///   * The `category` and `description` of every [`Expense`] of the `expenses` of the `timesheet`.
fn export_timesheet(timesheet: &Timesheet, output: &mut String)
{
	writeln!(output, "{}", Block::Heading {
		indents: 3,
		text: timesheet
			.time_end
			.map(|time_end| format!("{} – {}", timesheet.time_begin, time_end))
			.unwrap_or_else(|| format!("{} – Current", timesheet.time_begin)),
	})
	.unwrap();

	writeln!(
		output,
		"{}: {} {}",
		Block::UnorderedList {
			indents: 0,
			text: Text::Bold("Employee"),
		},
		timesheet.employee.title,
		timesheet.employee.name,
	)
	.unwrap();

	writeln!(output, "{}", Block::<&str>::Break).unwrap();

	if !timesheet.expenses.is_empty()
	{
		writeln!(output, "{}", Block::Heading {
			indents: 4,
			text: "Expenses",
		})
		.unwrap();

		timesheet
			.expenses
			.iter()
			.try_for_each(|e| {
				writeln!(
					output,
					"{}\n{}",
					Block::Heading {
						indents: 5,
						text: format!("№{} – {} ({})", e.id, e.category, e.cost),
					},
					Block::Text(&e.description),
				)
			})
			.unwrap();
	}

	if !timesheet.work_notes.is_empty()
	{
		writeln!(output, "{}", Block::Heading {
			indents: 4,
			text: "Work Notes",
		})
		.unwrap();
		writeln!(output, "{}", Block::Text(&timesheet.work_notes)).unwrap();
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;

	use clinvoice_finance::{Currency, Money};
	use clinvoice_schema::{
		chrono::Utc,
		Contact,
		ContactKind,
		Employee,
		Expense,
		Invoice,
		Location,
		Organization,
	};

	use super::{DateTime, Job, Local, Timesheet};

	#[test]
	fn export()
	{
		let client = Organization {
			id: 0,
			location: Location {
				id: 0,
				name: "1337 Some Street".into(),
				outer: Some(
					Location {
						id: 1,
						name: "Phoenix".into(),
						outer: Some(
							Location {
								id: 2,
								name: "Arizona".into(),
								outer: Some(
									Location {
										id: 3,
										name: "USA".into(),
										outer: Some(
											Location {
												id: 4,
												name: "Earth".into(),
												outer: None,
											}
											.into(),
										),
									}
									.into(),
								),
							}
							.into(),
						),
					}
					.into(),
				),
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
				kind: ContactKind::Other("TestyCo".into()),
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
				hourly_rate: Money::new(20_00, 2, Currency::Usd),
			},
			notes: "- I tested the function.".into(),
			objectives: "- I want to test this function.".into(),
		};

		assert_eq!(
			super::export_job(&job, &mut contact_info, None, &testy_organization, &[]),
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
					cost: Money::new(20_00, 2, Currency::Usd),
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
			super::export_job(
				&job,
				&mut contact_info,
				None,
				&testy_organization,
				&timesheets
			),
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
