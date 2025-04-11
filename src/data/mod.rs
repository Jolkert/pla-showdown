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

pub trait Identify
{
	fn id(&self) -> &str;
}

#[derive(Debug)]
struct Identifiable<T: Identify>(T);
impl<T: Identify> std::ops::Deref for Identifiable<T>
{
	type Target = T;

	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}
impl<T: Identify> std::borrow::Borrow<T> for Identifiable<T>
{
	fn borrow(&self) -> &T
	{
		&self.0
	}
}
impl<T: Identify> std::borrow::Borrow<str> for Identifiable<T>
{
	fn borrow(&self) -> &str
	{
		self.id()
	}
}

impl<T: Identify> PartialEq for Identifiable<T>
{
	fn eq(&self, other: &Self) -> bool
	{
		self.id() == other.id()
	}
}
impl<T: Identify> Eq for Identifiable<T> {}

impl<T: Identify> PartialOrd for Identifiable<T>
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
	{
		self.id().partial_cmp(other.id())
	}
}
impl<T: Identify> Ord for Identifiable<T>
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering
	{
		self.id().cmp(other.id())
	}
}

impl<T: Identify> std::hash::Hash for Identifiable<T>
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H)
	{
		self.id().hash(state)
	}
}

pub type RegMap<T> = HashMap<BoxStr, T>;
