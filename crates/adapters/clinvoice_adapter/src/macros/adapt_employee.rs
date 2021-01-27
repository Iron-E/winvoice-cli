#[macro_export]
macro_rules! AdaptEmployee
{
	($name: ident, $($life: lifetime)*, $($store_life: lifetime)*) =>
	{
		use clinvoice_adapter::Store;
		use clinvoice_data::Employee;

		/// # Summary
		///
		/// A wrapper around [`Employee`] with a [`Store`] that points to its location.
		#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
		pub struct $name<$($life),*, $($store_life),*>
		{
			pub employee: Employee<$($life),*>,
			pub store: Store<$($store_life),*>,
		}

		impl<$($life),*, $($store_life),*> Into<Employee<$($life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Employee<$($life),*>
			{
				return self.employee;
			}
		}

		impl<$($life),*, $($store_life),*> Into<Store<$($store_life),*>> for $name<$($life),*, $($store_life),*>
		{
			fn into(self) -> Store<$($store_life),*>
			{
				return self.store;
			}
		}
	};
}
