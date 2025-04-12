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
