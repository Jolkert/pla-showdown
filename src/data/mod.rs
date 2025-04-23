use std::{collections::HashMap, rc::Rc};

use crate::BoxStr;

pub mod serialization;
mod_pub_use_all! {
	moves,
	pokemon,
	stats,
	status,
	types,
}

#[derive(Debug)]
pub struct Data
{
	pub types: IdSet<Rc<Type>>,
	pub species: IdSet<Species>,
	pub moves: IdSet<Move>,
	pub statuses: IdSet<StatusCondition>,
	pub natures: HashMap<BoxStr, Nature>,
}

/// A trait for any data which can be given a unique (string) id
/// for use in conjunction with  `Identifiable<T>`. For this reason, any implementers
/// of this trait must be certain that no two values to be compared against one another
/// ever have the same ID, as all comparisons between `Identifiable`s are based
/// solely on the `&str` returned by the `Identify::id` method
pub trait Identify
{
	fn id(&self) -> &str;
}

impl<T> Identify for Rc<T>
where
	T: Identify,
{
	fn id(&self) -> &str
	{
		(**self).id()
	}
}

/// A newtype wrapper for any `Identify` type. Allows for equality, ordering, and hashing of
/// the inner type based solely on the value of its string id.
#[derive(Debug, Clone)]
pub struct Identifiable<T: Identify>(T);
impl<T: Identify> Identifiable<T>
{
	pub fn ref_inner(&self) -> &T
	{
		&self.0
	}
}

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
		Some(self.cmp(other))
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

impl<T: Identify> From<T> for Identifiable<T>
{
	fn from(value: T) -> Self
	{
		Identifiable(value)
	}
}

pub type IdSet<T> = std::collections::HashSet<Identifiable<T>>;
