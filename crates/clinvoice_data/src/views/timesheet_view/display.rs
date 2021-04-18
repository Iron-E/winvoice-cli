use
{
	core::fmt::{Display, Formatter, Result},

	super::TimesheetView,

	chrono::{DateTime, Local},
};

impl Display for TimesheetView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		writeln!(formatter, "{} – {}: {} {} from {}",
			DateTime::<Local>::from(self.time_begin),
			self.time_end.map(|time| DateTime::<Local>::from(time).to_string()).unwrap_or_else(|| "Current".into()),
			self.employee.title,
			self.employee.person.name,
			self.employee.organization,
		)?;

		const DEPTH_2: &str = "\n\t\t";

		if !self.expenses.is_empty()
		{
			writeln!(formatter, "\tExpenses:")?;
			self.expenses.iter().try_for_each(|e| writeln!(formatter, "\t\t{}", e.to_string().replace('\n', DEPTH_2)))?;
		}

		if !self.work_notes.is_empty()
		{
			write!(formatter, "\tWork Notes:{}{}", DEPTH_2, self.work_notes.replace('\n', DEPTH_2))?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::{collections::HashMap, time::Instant},

		super::{DateTime, Local, TimesheetView},
		crate::
		{
			Decimal, EmployeeStatus, Expense, ExpenseCategory, Id, Money,
			views::{ContactView, EmployeeView, LocationView, OrganizationView, PersonView}
		},

		chrono::Utc,
	};

	#[test]
	fn display()
	{
		let earth_view = LocationView
		{
			name: "Earth".into(),
			id: Id::new_v4(),
			outer: None,
		};

		let usa_view = LocationView
		{
			name: "USA".into(),
			id: Id::new_v4(),
			outer: Some(earth_view.into()),
		};

		let arizona_view = LocationView
		{
			name: "Arizona".into(),
			id: Id::new_v4(),
			outer: Some(usa_view.into())
		};

		let phoenix_view = LocationView
		{
			name: "Phoenix".into(),
			id: Id::new_v4(),
			outer: Some(arizona_view.into()),
		};

		let street_view = LocationView
		{
			name: "1337 Some Street".into(),
			id: Id::new_v4(),
			outer: Some(phoenix_view.into()),
		};

		let contact_info: HashMap<String, ContactView> = vec![
			("Street Address".into(), street_view.clone().into()),
			("Email".into(), ContactView::Email("foo@bar.io".into())),
			("Phone".into(), ContactView::Phone("1-800-555-5555".into())),
		].into_iter().collect();

		let timesheet =  TimesheetView
		{
			employee: EmployeeView
			{
				contact_info: contact_info.clone(),
				id: Id::new_v4(),
				organization: OrganizationView
				{
					id: Id::new_v4(),
					location: street_view,
					name: "Big Test Organization".into(),
				},
				person: PersonView
				{
					id: Id::new_v4(),
					name: "Testy McTesterson".into(),
				},
				status: EmployeeStatus::Representative,
				title: "CEO of Tests".into(),
			},
			expenses: vec![
				Expense
				{
					category: ExpenseCategory::Food,
					cost: Money::new(Decimal::new(2050, 2), "USD"),
					description: "Fast Food™".into(),
				},
				Expense
				{
					category: ExpenseCategory::Travel,
					cost: Money::new(Decimal::new(1000, 2), "USD"),
					description: "Gas".into(),
				},
			],
			time_begin: Utc::now(),
			time_end: Some(Utc::today().and_hms(23, 59, 59)),
			work_notes: "Went to non-corporate fast food restaurant for business meeting".into(),
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", timesheet),
			format!(
"{} – {}: CEO of Tests Testy McTesterson from Big Test Organization @ 1337 Some Street, Phoenix, Arizona, USA, Earth
	Expenses:
		Food – 20.50 USD
			Fast Food™
		Travel – 10.00 USD
			Gas
	Work Notes:
		Went to non-corporate fast food restaurant for business meeting",
				DateTime::<Local>::from(timesheet.time_begin),
				DateTime::<Local>::from(timesheet.time_end.unwrap()),
			),
		);
		println!("\n>>>>> TimesheetView::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
