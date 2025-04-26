use crate::BoxSlice;

// TODO: reanme all of the `Ser*` types. Its kinda a terrible name
// - morgan 2025-04-12
mod_pub_use_all! {
	moves,
	species,
	status,
	types,
}

fn empty_slice<T>() -> BoxSlice<T>
{
	Box::new([])
}

// TODO: this name also kinda sucks tbh
// -morgan 2025-04-12
pub trait IntoDeserialized<'a>
{
	type Deserialized;
	type RefData;

	fn into_deserialized(self, data: &'a Self::RefData) -> Self::Deserialized;
}
