use
{
	super::BincodeJob,
	clinvoice_adapter::DynamicResult,
	clinvoice_data::views::JobView,
};

impl Into<DynamicResult<JobView>> for BincodeJob<'_, '_, '_>
{
	fn into(self) -> DynamicResult<JobView>
	{
		todo!();
	}
}

