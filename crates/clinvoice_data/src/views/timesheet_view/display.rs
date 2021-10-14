use core::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Local};

use super::TimesheetView;

impl Display for TimesheetView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(
			formatter,
			"{} – {}: {} {} from {}",
			DateTime::<Local>::from(self.time_begin).naive_local(),
			self
				.time_end
				.map(|time| DateTime::<Local>::from(time).naive_local().to_string())
				.unwrap_or_else(|| "Current".into()),
			self.employee.title,
			self.employee.person.name,
			self.employee.organization,
		)?;

		const DEPTH_2: &str = "\n\t\t";

		if !self.expenses.is_empty()
		{
			writeln!(formatter, "\tExpenses:")?;
			self.expenses.iter().try_for_each(|e| {
				writeln!(formatter, "\t\t{}", e.to_string().replace('\n', DEPTH_2))
			})?;
		}

		if !self.work_notes.is_empty()
		{
			write!(
				formatter,
				"\tWork Notes:{}{}",
				DEPTH_2,
				self.work_notes.replace('\n', DEPTH_2)
			)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use std::{collections::HashMap, time::Instant};

	use chrono::Utc;
	use clinvoice_finance::{Currency, Money};

	use super::{DateTime, Local, TimesheetView};
	use crate::{
		views::{ContactView, EmployeeView, LocationView, OrganizationView, PersonView},
		EmployeeStatus,
		Expense,
		ExpenseCategory,
	};

	#[test]
	fn display()
	{
		let earth_view = LocationView {
			id:    0,
			name:  "Earth".into(),
			outer: None,
		};

		let usa_view = LocationView {
			id:    0,
			name:  "USA".into(),
			outer: Some(earth_view.into()),
		};

		let arizona_view = LocationView {
			id:    0,
			name:  "Arizona".into(),
			outer: Some(usa_view.into()),
		};

		let phoenix_view = LocationView {
			id:    0,
			name:  "Phoenix".into(),
			outer: Some(arizona_view.into()),
		};

		let street_view = LocationView {
			id:    0,
			name:  "1337 Some Street".into(),
			outer: Some(phoenix_view.into()),
		};

		let contact_info: HashMap<String, ContactView> = vec![
			("Street Address".into(), ContactView::Address {
				location: street_view.clone(),
				export:   false,
			}),
			("Email".into(), ContactView::Email {
				email:  "foo@bar.io".into(),
				export: false,
			}),
			("Phone".into(), ContactView::Phone {
				phone:  "1-800-555-5555".into(),
				export: false,
			}),
		]
		.into_iter()
		.collect();

		let timesheet = TimesheetView {
			employee:   EmployeeView {
				contact_info: contact_info.clone(),
				id: 0,
				organization: OrganizationView {
					id: 0,
					location: street_view,
					name: "Big Test Organization".into(),
				},
				person: PersonView {
					id:   0,
					name: "Testy McTesterson".into(),
				},
				status: EmployeeStatus::Representative,
				title: "CEO of Tests".into(),
			},
			expenses:   vec![
				Expense {
					category: ExpenseCategory::Food,
					cost: Money::new(20_50, 2, Currency::USD),
					description: "Fast Food™".into(),
				},
				Expense {
					category: ExpenseCategory::Travel,
					cost: Money::new(10_00, 2, Currency::USD),
					description: "Gas".into(),
				},
			],
			job_id:     0,
			time_begin: Utc::now(),
			time_end:   Some(Utc::today().and_hms(23, 59, 59)),
			work_notes: "Went to non-corporate fast food restaurant for business meeting".into(),
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", timesheet),
			format!(
				"{} – {}: CEO of Tests Testy McTesterson from Big Test Organization @ 1337 Some \
				 Street, Phoenix, Arizona, USA, Earth
	Expenses:
		Food – 20.50 USD
			Fast Food™
		Travel – 10.00 USD
			Gas
	Work Notes:
		Went to non-corporate fast food restaurant for business meeting",
				DateTime::<Local>::from(timesheet.time_begin).naive_local(),
				DateTime::<Local>::from(timesheet.time_end.unwrap()).naive_local(),
			),
		);
		println!(
			"\n>>>>> TimesheetView::fmt {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros()
		);
	}
}
