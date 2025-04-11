use crate::BoxSlice;

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
