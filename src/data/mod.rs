pub mod serialization;
mod_pub_use_all! {
	moves,
	pokemon,
	stats,
	status,
	types,
}

use std::collections::HashMap;

pub trait Identifiable
{
	fn id(&self) -> Box<str>;
}

pub type RegMap<T> = HashMap<Box<str>, T>;
