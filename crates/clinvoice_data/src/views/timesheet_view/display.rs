use
{
	super::TimesheetView,
	std::fmt::{Display, Formatter, Result},
};

impl Display for TimesheetView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		writeln!(formatter, "{} from {}: {} – {}",
			self.employee.person.name,
			self.employee.organization.name,
			self.time_begin,
			match self.time_end
			{
				Some(time) => time.to_string(),
				_ => "Current".into(),
			},
		)?;

		if let Some(expenses) = &self.expenses
		{
			writeln!(formatter, "\nExpenses:")?;
			expenses.iter().try_for_each(|e| writeln!(formatter, "\t{}", e.to_string().replace('\n', "\n\t")))?;
		}

		return write!(formatter, "\nWork Notes:\n\t{}", self.work_notes.replace('\n', "\n\t"));
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::TimesheetView,
		crate::
		{
			chrono::Utc, Decimal, EmployeeStatus, Expense, ExpenseCategory, Id, Money,
			views::{ContactView, EmployeeView, LocationView, OrganizationView, PersonView}
		},
		std::time::Instant,
	};

	#[test]
	fn test_display()
	{
		let start = Instant::now();

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

		let contact_info = vec![
			ContactView::Address(street_view.clone()),
			ContactView::Email("foo@bar.io".into()),
			ContactView::Phone("1-800-555-5555".into()),
		];

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
					contact_info,
					id: Id::new_v4(),
					name: "Testy McTesterson".into(),
				},
				status: EmployeeStatus::Representative,
				title: "CEO of Tests".into(),
			},
			expenses: Some(vec![
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
			]),
			time_begin: Utc::now(),
			time_end: Some(Utc::today().and_hms(23, 59, 59)),
			work_notes: "Went to non-corporate fast food restaurant for business meeting.".into(),
		};

		assert_eq!(
			format!("{}", timesheet),
			format!(
"Testy McTesterson from Big Test Organization: {} – {}

Expenses:
	Food – 20.50 USD
		Fast Food™
	Travel – 10.00 USD
		Gas

Work Notes:
	Went to non-corporate fast food restaurant for business meeting.",
				timesheet.time_begin,
				timesheet.time_end.unwrap()
			),
		);

		println!("\n>>>>> TimesheetView test_display {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
