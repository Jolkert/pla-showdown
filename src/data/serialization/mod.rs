mod_pub_use_all! {
	moves,
	species,
	status,
}

fn empty_slice<T>() -> Box<[T]>
{
	Box::new([])
}
