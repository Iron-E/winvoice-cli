use
{
	core::fmt::{Display, Formatter, Result},

	super::JobView,
	crate::Job,

	chrono::{DateTime, Local},
};

impl Display for JobView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(formatter, "Job #{} for {}: {} – ",
			self.id,
			self.client.name,
			DateTime::<Local>::from(self.date_open),
		)?;

		if let Some(date) = self.date_close
		{
			writeln!(formatter, "{}", DateTime::<Local>::from(date))?;
		}
		else
		{
			writeln!(formatter, "Current")?;
		}

		/// # Summary
		///
		/// Two indents in, with a newline.
		const DEPTH_2: &str =  "\n\t\t";

		writeln!(formatter, "\tInvoice:{}{}", DEPTH_2, self.invoice.to_string().replace('\n', DEPTH_2))?;
		writeln!(formatter, "\t\tTotal Amount Owed: {}", Job::from(self).total())?;

		if !self.objectives.is_empty()
		{
			writeln!(formatter, "\tObjectives:{}{}", DEPTH_2, self.objectives.replace('\n', DEPTH_2))?;
		}

		if !self.notes.is_empty()
		{
			writeln!(formatter, "\tNotes:{}{}", DEPTH_2, self.notes.replace('\n', DEPTH_2))?;
		}

		if !self.timesheets.is_empty()
		{
			write!(formatter, "\tTimesheets:")?;
			self.timesheets.iter().try_for_each(|t| write!(formatter, "{}{}", DEPTH_2, t.to_string().replace('\n', DEPTH_2)))?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use
	{
		std::time::Instant,

		super::{DateTime, JobView, Local},
		crate::
		{
			Decimal, EmployeeStatus, Id, Invoice, Job, Money,
			views::{ContactView, EmployeeView, LocationView, OrganizationView, PersonView, TimesheetView},
		},

		chrono::Utc,
	};

	#[test]
	fn display()
	{
		let earth_view = LocationView
		{
			id: Id::new_v4(),
			name: "Earth".into(),
			outer: None,
		};

		let ceo_testy_view = EmployeeView
		{
			contact_info: vec![("Office".into(), ContactView::Address(earth_view.clone()))].into_iter().collect(),
			id: Id::new_v4(),
			organization: OrganizationView
			{
				id: Id::new_v4(),
				location: earth_view.clone(),
				name: "Big Old Test".into(),
			},
			person: PersonView
			{
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
			objectives: "Get into the mainframe, or something like that".into(),
			timesheets: vec![TimesheetView
			{
				employee: ceo_testy_view,
				expenses: Vec::new(),
				time_begin: Utc::now(),
				time_end: Some(Utc::today().and_hms(23, 59, 59)),
				work_notes: "Went to non-corporate fast food restaurant for business meeting".into(),
			}],
		};

		let start = Instant::now();
		assert_eq!(
			format!("{}", create_job_view),
			format!(
"Job #{} for Big Old Test: {} – {}
	Invoice:
		Hourly Rate: 20.00 USD
		Status: Not issued
		Total Amount Owed: {}
	Objectives:
		Get into the mainframe, or something like that
	Notes:
		Remember not to work with these guys again!
	Timesheets:
		{} – {}: CEO of Tests Testy McTesterson from Big Old Test @ Earth
			Work Notes:
				Went to non-corporate fast food restaurant for business meeting",
				create_job_view.id,
				DateTime::<Local>::from(create_job_view.date_open),
				DateTime::<Local>::from(create_job_view.date_close.unwrap()),
				Job::from(&create_job_view).total(),
				DateTime::<Local>::from(create_job_view.timesheets.first().unwrap().time_begin),
				DateTime::<Local>::from(create_job_view.timesheets.first().unwrap().time_end.unwrap()),
			),
		);
		println!("\n>>>>> JobView::fmt {}us <<<<<\n", Instant::now().duration_since(start).as_micros());
	}
}
