use super::Job;

impl PartialEq for Job
{
	fn eq(&self, other: &Self) -> bool
	{
		self.id == other.id
	}
}
