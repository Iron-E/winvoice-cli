use core::fmt::{Display, Error, Formatter, Result};

use chrono::{DateTime, Local};

use super::JobView;

impl Display for JobView
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		write!(
			formatter,
			"Job #{} for {}: {} – ",
			self.id,
			self.client.name,
			DateTime::<Local>::from(self.date_open).naive_local(),
		)?;

		if let Some(date) = self.date_close
		{
			writeln!(formatter, "{}", DateTime::<Local>::from(date).naive_local())?;
		}
		else
		{
			writeln!(formatter, "Current")?;
		}

		/// # Summary
		///
		/// Two indents in, with a newline.
		const DEPTH_2: &str = "\n\t\t";

		writeln!(
			formatter,
			"\tInvoice:{}{}",
			DEPTH_2,
			self.invoice.to_string().replace('\n', DEPTH_2)
		)?;
		let total = self.total().map_err(|_| Error)?;
		writeln!(formatter, "\t\tTotal Amount Owed: {}", total)?;

		if !self.objectives.is_empty()
		{
			writeln!(
				formatter,
				"\tObjectives:{}{}",
				DEPTH_2,
				self.objectives.replace('\n', DEPTH_2)
			)?;
		}

		if !self.notes.is_empty()
		{
			writeln!(
				formatter,
				"\tNotes:{}{}",
				DEPTH_2,
				self.notes.replace('\n', DEPTH_2)
			)?;
		}

		if !self.timesheets.is_empty()
		{
			write!(formatter, "\tTimesheets:")?;
			self.timesheets.iter().try_for_each(|t| {
				write!(
					formatter,
					"{}{}",
					DEPTH_2,
					t.to_string().replace('\n', DEPTH_2)
				)
			})?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests
{
	use core::time::Duration;
	use std::time::Instant;

	use chrono::Utc;
	use clinvoice_finance::{Currency, Money};

	use super::{DateTime, JobView, Local};
	use crate::{
		views::{
			ContactView,
			EmployeeView,
			LocationView,
			OrganizationView,
			PersonView,
			TimesheetView,
		},
		EmployeeStatus,
		Invoice,
	};

	#[cfg(uuid)]
	use crate::Id;

	#[test]
	fn display()
	{
		let earth_view = LocationView {
			#[cfg(uuid)]
			id:    Id::new_v4(),
			#[cfg(not(uuid))]
			id:    0,
			name:  "Earth".into(),
			outer: None,
		};

		let ceo_testy_view = EmployeeView {
			contact_info: vec![("Office".into(), ContactView::Address {
				location: earth_view.clone(),
				export:   false,
			})]
			.into_iter()
			.collect(),
			#[cfg(uuid)]
			id:    Id::new_v4(),
			#[cfg(not(uuid))]
			id:    0,
			organization: OrganizationView {
				#[cfg(uuid)]
				id:    Id::new_v4(),
				#[cfg(not(uuid))]
				id:    0,
				location: earth_view.clone(),
				name: "Big Old Test".into(),
			},
			person: PersonView {
				#[cfg(uuid)]
				id:    Id::new_v4(),
				#[cfg(not(uuid))]
				id:    0,
				name: "Testy McTesterson".into(),
			},
			status: EmployeeStatus::Representative,
			title: "CEO of Tests".into(),
		};

		let mut create_job_view = JobView {
			client: ceo_testy_view.organization.clone(),
			date_close: Some(Utc::today().and_hms(23, 59, 59)),
			date_open: Utc::now(),
			#[cfg(uuid)]
			id:    Id::new_v4(),
			#[cfg(not(uuid))]
			id:    0,
			interval: Duration::from_secs(900),
			invoice: Invoice {
				date: None,
				hourly_rate: Money::new(20_00, 2, Currency::USD),
			},
			notes: "Remember not to work with these guys again!".into(),
			objectives: "Get into the mainframe, or something like that".into(),
			timesheets: Vec::new(),
		};

		create_job_view.timesheets.push(TimesheetView {
			employee:   ceo_testy_view,
			expenses:   Vec::new(),
			job_id: create_job_view.id,
			time_begin: Utc::now(),
			time_end:   Some(Utc::today().and_hms(23, 59, 59)),
			work_notes: "Went to non-corporate fast food restaurant for business meeting".into(),
		});

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
				DateTime::<Local>::from(create_job_view.date_open).naive_local(),
				DateTime::<Local>::from(create_job_view.date_close.unwrap()).naive_local(),
				&create_job_view.total().unwrap(),
				DateTime::<Local>::from(create_job_view.timesheets.first().unwrap().time_begin)
					.naive_local(),
				DateTime::<Local>::from(
					create_job_view
						.timesheets
						.first()
						.unwrap()
						.time_end
						.unwrap()
				)
				.naive_local(),
			),
		);
		println!(
			"\n>>>>> JobView::fmt {}us <<<<<\n",
			Instant::now().duration_since(start).as_micros()
		);
	}
}
