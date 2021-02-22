use
{
	super::Timesheet,
	crate::views::TimesheetView as View,
};

impl From<View> for Timesheet
{
	fn from(view: View) -> Self
	{
		return Self
		{
			employee_id: view.employee.id,
			expenses: view.expenses,
			time_begin: view.time_begin,
			time_end: view.time_end,
			work_notes: view.work_notes,
		};
	}
}


impl From<&View> for Timesheet
{
	fn from(view: &View) -> Self
	{
		return Self
		{
			employee_id: view.employee.id,
			expenses: view.expenses.clone(),
			time_begin: view.time_begin,
			time_end: view.time_end,
			work_notes: view.work_notes.clone(),
		};
	}
}

