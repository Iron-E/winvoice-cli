mod util;

use
{
	clinvoice_adapter::data::LocationAdapter,
	clinvoice_adapter_bincode::data::BincodeLocation,
	clinvoice_data::views::LocationView,
	std::time::Instant,
};

#[test]
fn into_view()
{
	util::temp_store(|store|
	{
		let earth = BincodeLocation::create("Earth", &store).unwrap();

		let usa = BincodeLocation
		{
			location: &earth,
			store,
		}.create_inner("USA").unwrap();

		let arizona = BincodeLocation
		{
			location: &usa,
			store,
		}.create_inner("Arizona").unwrap();

		let phoenix = BincodeLocation
		{
			location: &arizona,
			store,
		}.create_inner("Phoenix").unwrap();

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
		let phoenix_view_result = BincodeLocation::into_view(phoenix, store);
		println!("\n>>>>> BincodeLocation::into_view {}us <<<<<\n", Instant::now().duration_since(start).as_micros());

		assert_eq!(phoenix_view, phoenix_view_result.unwrap());
	});
}
