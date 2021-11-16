use super::Job;
use crate::views::JobView as View;

impl From<View> for Job
{
	fn from(view: View) -> Self
	{
		Self {
			client_id: view.client.id,
			date_close: view.date_close,
			date_open: view.date_open,
			id: view.id,
			increment: view.increment,
			invoice: view.invoice,
			notes: view.notes,
			objectives: view.objectives,
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
			increment: view.increment,
			invoice: view.invoice.clone(),
			notes: view.notes.clone(),
			objectives: view.objectives.clone(),
		}
	}
}