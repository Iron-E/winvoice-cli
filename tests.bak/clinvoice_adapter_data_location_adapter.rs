mod util;

use std::time::Instant;

use clinvoice_adapter::data::LocationAdapter;
use clinvoice_adapter_bincode::data::BincodeLocation;
use clinvoice_data::views::LocationView;

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn into_view()
{
	let store = util::temp_store();

	let earth = BincodeLocation::create("Earth".into(), &store)
		.await
		.unwrap();

	let usa = BincodeLocation {
		location: &earth,
		store:    &store,
	}
	.create_inner("USA".into())
	.await
	.unwrap();

	let arizona = BincodeLocation {
		location: &usa,
		store:    &store,
	}
	.create_inner("Arizona".into())
	.await
	.unwrap();

	let phoenix = BincodeLocation {
		location: &arizona,
		store:    &store,
	}
	.create_inner("Phoenix".into())
	.await
	.unwrap();

	let phoenix_view = LocationView {
		id:    phoenix.id,
		name:  phoenix.name.clone(),
		outer: Some(
			LocationView {
				id:    arizona.id,
				name:  arizona.name,
				outer: Some(
					LocationView {
						id:    usa.id,
						name:  usa.name,
						outer: Some(
							LocationView {
								id:    earth.id,
								name:  earth.name,
								outer: None,
							}
							.into(),
						),
					}
					.into(),
				),
			}
			.into(),
		),
	};

	let start = Instant::now();
	let phoenix_view_result = BincodeLocation::into_view(phoenix, &store).await;
	println!(
		"\n>>>>> BincodeLocation::into_view {}us <<<<<\n",
		Instant::now().duration_since(start).as_micros()
	);

	assert_eq!(phoenix_view, phoenix_view_result.unwrap());
}