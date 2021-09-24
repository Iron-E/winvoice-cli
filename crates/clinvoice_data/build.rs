fn main()
{
	cfg_aliases::cfg_aliases! {
		uuid: { any(feature = "serde_support_unique_id", feature = "unique_id") },
	}
}
