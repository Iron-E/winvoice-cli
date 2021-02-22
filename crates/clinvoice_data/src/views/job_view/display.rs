use
{
	super::JobView,
	crate::Job,
	std::fmt::{Display, Formatter, Result},
};

impl Display for JobView
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result
	{
		writeln!(formatter, "Job #{} for {}: {} – {}",
			self.id,
			self.client.name,
			self.date_open,
			match self.date_close
			{
				Some(date) => date.to_string(),
				_ => "Current".into(),
			},
		)?;

		writeln!(formatter, "\t{}", self.invoice.to_string().replace('\n', "\n\t"))?;
		writeln!(formatter, "\tNotes:\n\t\t{}", self.notes.replace('\n', "\n\t\t"))?;
		writeln!(formatter, "\tObjectives:\n\t\t{}", self.objectives.replace('\n', "\n\t\t"))?;
		writeln!(formatter, "\tTimesheets:")?;
		self.timesheets.iter().try_for_each(|t| writeln!(formatter, "\t\t{}", t.to_string().replace('\n', "\n\t\t")))?;

		return write!(formatter, "\tTotal Amount Owed: {}", Job::from(self).total());
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		super::JobView,
		crate::
		{
			chrono::Utc, Decimal, EmployeeStatus, Id, Invoice, Job, Money,
			views::{ContactView, EmployeeView, LocationView, OrganizationView, PersonView, TimesheetView},
		},
		std::time::Instant,
	};

	#[test]
	fn test_display()
	{
		let earth_view = LocationView
		{
			id: Id::new_v4(),
			name: "Earth".into(),
			outer: None,
		};

		let contact_info_view = vec![ContactView::Address(earth_view.clone())];

		let ceo_testy_view = EmployeeView
		{
			contact_info: contact_info_view.clone(),
			id: Id::new_v4(),
			organization: OrganizationView
			{
				id: Id::new_v4(),
				location: earth_view,
				name: "Big Old Test".into(),
			},
			person: PersonView
			{
				contact_info: contact_info_view,
				id: Id::new_v4(),
				name: "Testy McTesterson".into(),
			},
			status: EmployeeStatus::Representative,
			title: "CEO of Tests".into(),
		};

		let create_job_view = JobView
		{
			client: ceo_testy_view.organization.clone(),
			date_close: Some(Utc::today().and_hms(23, 59, 59)),
			date_open: Utc::now(),
			id: Id::new_v4(),
			invoice: Invoice
			{
				date: None,
				hourly_rate: Money::new(Decimal::new(2000, 2), "USD"),
			},
			notes: "Remember not to work with these guys again!".into(),
			objectives: "Get into the mainframe, or something like that.".into(),
			timesheets: vec![TimesheetView
			{
				employee: ceo_testy_view,
				expenses: None,
				time_begin: Utc::now(),
				time_end: Some(Utc::today().and_hms(23, 59, 59)),
				work_notes: "Went to non-corporate fast food restaurant for business meeting.".into(),
			}],
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", create_job_view),
			format!(
"Job #{} for Big Old Test: {} – {}
	Hourly Rate: 20.00 USD
	Invoice Status: Not issued.
	Notes:
		Remember not to work with these guys again!
	Objectives:
		Get into the mainframe, or something like that.
	Timesheets:
		CEO of Tests Testy McTesterson from Big Old Test: {} – {}
			Work Notes:
				Went to non-corporate fast food restaurant for business meeting.
	Total Amount Owed: {}",
				create_job_view.id,
				create_job_view.date_open,
				create_job_view.date_close.unwrap(),
				create_job_view.timesheets.first().unwrap().time_begin,
				create_job_view.timesheets.first().unwrap().time_end.unwrap(),
				Job::from(&create_job_view).total(),
			),
		);
		println!("\n>>>>> JobView::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
