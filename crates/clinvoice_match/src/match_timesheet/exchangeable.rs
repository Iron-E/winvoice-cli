use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::MatchTimesheet;

impl Exchangeable for MatchTimesheet
{
	/// # Summary
	///
	/// Exchange a the `cost` to another `currency`.
	fn exchange(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			id: self.id.clone(),
			employee: self.employee.clone(),
			expenses: self.expenses.exchange(currency, rates),
			job: self.job.exchange(currency, rates),
			time_begin: self.time_begin.clone(),
			time_end: self.time_end.clone(),
			work_notes: self.work_notes.clone(),
		}
	}
}
