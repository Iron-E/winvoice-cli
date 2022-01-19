mod util;

use clinvoice_adapter::schema::LocationAdapter;
use clinvoice_adapter_bincode::schema::BincodeLocation;
use clinvoice_schema::Location;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
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

	let phoenix_view = Location {
		id:    phoenix.id,
		name:  phoenix.name.clone(),
		outer: Some(
			Location {
				id:    arizona.id,
				name:  arizona.name,
				outer: Some(
					Location {
						id:    usa.id,
						name:  usa.name,
						outer: Some(
							Location {
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

	let phoenix_view_result = BincodeLocation::into_view(phoenix, &store).await;

	assert_eq!(phoenix_view, phoenix_view_result.unwrap());
}
