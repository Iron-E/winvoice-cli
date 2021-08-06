use super::Job;
use crate::views::{JobView as View, TimesheetView};

impl From<View> for Job
{
	fn from(view: View) -> Self
	{
		Self {
			client_id: view.client.id,
			date_close: view.date_close,
			date_open: view.date_open,
			id: view.id,
			invoice: view.invoice,
			notes: view.notes,
			objectives: view.objectives,
			timesheets: view.timesheets.into_iter().map(TimesheetView::into).collect(),
		}
	}
}

impl From<&View> for Job
{
	fn from(view: &View) -> Self
	{
		Self {
			client_id: view.client.id,
			date_close: view.date_close,
			date_open: view.date_open,
			id: view.id,
			invoice: view.invoice.clone(),
			notes: view.notes.clone(),
			objectives: view.objectives.clone(),
			timesheets: view.timesheets.iter().cloned().map(TimesheetView::into).collect(),
		}
	}
}
