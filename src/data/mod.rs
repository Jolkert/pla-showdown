pub mod serialization;
mod_pub_use_all! {
	moves,
	pokemon,
	stats,
	status,
	types,
}

use std::collections::HashMap;

use crate::BoxStr;

pub trait Identifiable
{
	fn id(&self) -> BoxStr;
}

pub type RegMap<T> = HashMap<BoxStr, T>;
