mod error;
mod from_str;

pub use error::{Error, Result};

use
{
	std::collections::HashSet,
	core::fmt::Write,

	crate::markdown,

	clinvoice_data::
	{
		chrono::{DateTime, Local},
		Id, Job,
		views::{ContactView, JobView, TimesheetView},
	},
};

/// # Summary
///
/// A target for exporting.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Target
{
	/// # Summary
	///
	/// The markdown target. Exports to a `.md` file.
	#[cfg(feature="markdown")]
	Markdown,
}

impl Target
{
	/// # Summary
	///
	/// Export some `job` to the [`Target`] specified. Appends to some pre-existing `output`, in
	/// case multiple [`TimesheetView`]s must be exported sequentially.
	///
	/// Tracks the previously `exported_employees` so that their contact information is not
	/// reiterated every time.
	fn export_timesheet(&self, exported_employees: &mut HashSet<Id>, output: &mut String, timesheet: &TimesheetView)
	{
		match self
		{
			#[cfg(feature="markdown")]
			Self::Markdown =>
			{

				writeln!(output, "{}", markdown::Element::Heading
				{
					depth: 3,
					text: timesheet.time_end.map(|time_end|
						format!("{} – {}", timesheet.time_begin, time_end.naive_local())
					).unwrap_or_else(||
						format!("{} – Current", timesheet.time_begin)
					),
				}).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Employee Information"}).unwrap();
				writeln!(output, "{}: {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Name")},
					timesheet.employee.person.name,
				).unwrap();
				writeln!(output, "{}: {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Employer")},
					timesheet.employee.organization,
				).unwrap();
				writeln!(output, "{}: {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Title")},
					timesheet.employee.title,
				).unwrap();

				if exported_employees.contains(&timesheet.employee.id)
				{
					let employee_contact_info: Vec<_> = timesheet.employee.contact_info.iter().filter(|(_, c)| match c
					{
						ContactView::Address {location: _, export} => *export,
						ContactView::Email {email: _, export} => *export,
						ContactView::Phone {phone: _, export} => *export,
					}).collect();

					if !employee_contact_info.is_empty()
					{
						writeln!(output, "{}:", markdown::Element::UnorderedList
						{
							depth: 0,
							text: markdown::Text::Bold("Contact Information"),
						}).unwrap();

						let mut sorted_employee_contact_info = employee_contact_info;
						sorted_employee_contact_info.sort_by_key(|(label, _)| *label);

						sorted_employee_contact_info.into_iter().try_for_each(|(label, contact)| writeln!(output, "{}: {}",
							markdown::Element::UnorderedList {depth: 1, text: markdown::Text::Bold(label)},
							contact,
						)).unwrap();
					}
				}

				writeln!(output, "{}", markdown::Element::<&str>::Break).unwrap();

				if !timesheet.expenses.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Expenses"}).unwrap();

					timesheet.expenses.iter().try_for_each(|e| writeln!(output, "{}\n{}",
						markdown::Element::Heading {depth: 5, text: format!("{} – {}", e.category, e.cost)},
						markdown::Element::BlockText(&e.description),
					)).unwrap();
				}

				if !timesheet.work_notes.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 4, text: "Work Notes"}).unwrap();
					writeln!(output, "{}", markdown::Element::BlockText(&timesheet.work_notes)).unwrap();
				}
			},
		};

		exported_employees.insert(timesheet.employee.id);
	}

	/// # Summary
	///
	/// Export some `job` to the [`Target`] specified.
	pub fn export_job(&self, job: &JobView) -> String
	{
		let mut output = String::new();

		match self
		{
			#[cfg(feature="markdown")]
			Self::Markdown =>
			{
				writeln!(output, "{}", markdown::Element::Heading {depth: 1, text: format!("Job #{}", job.id)}).unwrap();

				writeln!(output, "{}: {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Client")},
					job.client,
				).unwrap();

				writeln!(output, "{}: {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Date Opened")},
					DateTime::<Local>::from(job.date_open),
				).unwrap();

				if let Some(date) = job.date_close
				{
					writeln!(output, "{}: {}",
						markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Date Closed")},
						DateTime::<Local>::from(date).naive_local(),
					).unwrap();
				}

				writeln!(output, "{}", markdown::Element::<&str>::Break).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Invoice"}).unwrap();
				writeln!(output, "{} {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Hourly Rate")},
					job.invoice.hourly_rate,
				).unwrap();

				if let Some(date) = &job.invoice.date
				{
					writeln!(output, "{}: {}",
						markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Status")},
						date,
					).unwrap();
				}

				writeln!(output, "{}: {}",
					markdown::Element::UnorderedList {depth: 0, text: markdown::Text::Bold("Total Amount Owed")},
					Job::from(job).total(),
				).unwrap();
				writeln!(output, "{}", markdown::Element::<&str>::Break).unwrap();

				writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Objectives"}).unwrap();
				writeln!(output, "{}", markdown::Element::BlockText(&job.objectives)).unwrap();

				if !job.notes.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Notes"}).unwrap();
					writeln!(output, "{}", markdown::Element::BlockText(&job.notes)).unwrap();
				}

				if !job.timesheets.is_empty()
				{
					writeln!(output, "{}", markdown::Element::Heading {depth: 2, text: "Timesheets"}).unwrap();
					let mut employees = HashSet::new();
					job.timesheets.iter().for_each(|t| self.export_timesheet(&mut employees, &mut output, t));
				}
			},
		};

		output
	}

	/// # Summary
	///
	/// Return the file extension of an export [`Target`].
	pub fn extension(&self) -> &'static str
	{
		match self
		{
			Self::Markdown => ".md",
		}
	}
}

#[cfg(all(feature="markdown", test))]
mod tests
{
	use
	{
		std::{collections::HashMap, time::Instant},

		super::{JobView, Target, TimesheetView},

		clinvoice_data::
		{
			chrono::{DateTime, Local, Utc},
			Decimal, EmployeeStatus, Expense, ExpenseCategory, Id, Invoice, Money,
			views::{EmployeeView, LocationView, OrganizationView, PersonView},
		},
	};

	#[test]
	fn export_job()
	{
		let organization = OrganizationView
		{
			id: Id::new_v4(),
			location: LocationView
			{
				id: Id::new_v4(),
				outer: Some(LocationView
				{
					id: Id::new_v4(),
					outer: Some(LocationView
					{
						id: Id::new_v4(),
						outer: Some(LocationView
						{
							id: Id::new_v4(),
							outer: Some(LocationView
							{
								id: Id::new_v4(),
								outer: None,
								name: "Earth".into(),
							}.into()),
							name: "USA".into(),
						}.into()),
						name: "Arizona".into(),
					}.into()),
					name: "Phoenix".into(),
				}.into()),
				name: "1337 Some Street".into(),
			},
			name: "Big Old Test".into(),
		};

		let testy_mctesterson = EmployeeView
		{
			contact_info: vec![
			].into_iter().collect(),
			id: Id::new_v4(),
			organization: organization.clone(),
			person: PersonView
			{
				id: Id::new_v4(),
				name: "Testy McTesterson".into(),
			},
			status: EmployeeStatus::Representative,
			title: "CEO of Tests".into(),
		};

		let bob = EmployeeView
		{
			contact_info: HashMap::new(),
			id: Id::new_v4(),
			organization: organization.clone(),
			person: PersonView
			{
				id: Id::new_v4(),
				name: "Bob".into(),
			},
			status: EmployeeStatus::Employed,
			title: "Janitor".into(),
		};

		let mut job = JobView
		{
			client: organization,
			date_close: None,
			date_open: Utc::today().and_hms(0, 0, 0),
			id: Id::new_v4(),
			invoice: Invoice
			{
				date: None,
				hourly_rate: Money::new(Decimal::new(2000, 2), "USD"),
			},
			notes: "- I tested the function.".into(),
			objectives: "- I want to test this function.".into(),
			timesheets: vec![],
		};

		let start = Instant::now();
		assert_eq!(
			Target::Markdown.export_job(&job),
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
		let middle = Instant::now().duration_since(start);

		job.date_close = Some(Utc::today().and_hms(4, 30, 0));

		job.timesheets = vec![
			TimesheetView
			{
				employee: testy_mctesterson,
				expenses: Vec::new(),
				time_begin: Utc::today().and_hms(2, 0, 0),
				time_end: Some(Utc::today().and_hms(2, 30, 0)),
				work_notes: "- Wrote the test.".into(),
			},
			TimesheetView
			{
				employee: bob,
				expenses: vec![
					Expense
					{
						category: ExpenseCategory::Service,
						cost: Money::new(Decimal::new(2000, 2), "USD"),
						description: "Paid for someone else to clean".into(),
					},
				],
				time_begin: Utc::today().and_hms(3, 0, 0),
				time_end: Some(Utc::today().and_hms(3, 30, 0)),
				work_notes: "- Clean the deck.".into(),
			},
		];

		let second_start = Instant::now();
		assert_eq!(
			Target::Markdown.export_job(&job),
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

##### Service – 20.00 USD

Paid for someone else to clean

#### Work Notes

- Clean the deck.\n\n",
				job.id,
				DateTime::<Local>::from(job.date_open),
				DateTime::<Local>::from(job.date_close.unwrap()).naive_local(),
				job.timesheets[0].time_begin,
				job.timesheets[0].time_end.unwrap().naive_local(),
				job.timesheets[1].time_begin,
				job.timesheets[1].time_end.unwrap().naive_local(),
			),
		);
		println!("\n>>>>> Target::Markdown.export_job {}us <<<<<\n", (Instant::now().duration_since(second_start) + middle).as_micros());
	}
}
