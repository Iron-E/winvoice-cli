mod util;

use
{
	std::time::Instant,

	clinvoice_adapter::data::LocationAdapter,
	clinvoice_adapter_bincode::data::BincodeLocation,
	clinvoice_data::views::LocationView,
};

#[tokio::test(flavor="multi_thread", worker_threads=10)]
async fn to_view()
{
	let store = util::temp_store();

	let earth = BincodeLocation::create("Earth".into(), &store).await.unwrap();

	let usa = BincodeLocation
	{
		location: &earth,
		store,
	}.create_inner("USA".into()).unwrap();

	let arizona = BincodeLocation
	{
		location: &usa,
		store,
	}.create_inner("Arizona".into()).unwrap();

	let phoenix = BincodeLocation
	{
		location: &arizona,
		store,
	}.create_inner("Phoenix".into()).unwrap();

	let phoenix_view = LocationView
	{
		id: phoenix.id,
		name: phoenix.name.clone(),
		outer: Some(LocationView
		{
			id: arizona.id,
			name: arizona.name,
			outer: Some(LocationView
			{
				id: usa.id,
				name: usa.name,
				outer: Some(LocationView
				{
					id: earth.id,
					name: earth.name,
					outer: None,
				}.into()),
			}.into()),
		}.into()),
	};

	let start = Instant::now();
	let phoenix_view_result = BincodeLocation::into_view(phoenix, store).await;
	println!("\n>>>>> BincodeLocation::to_view {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

	assert_eq!(phoenix_view, phoenix_view_result.unwrap());
}
