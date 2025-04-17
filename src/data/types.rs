use std::rc::{Rc, Weak};

use crate::{BoxSlice, data::Identify};

#[derive(Debug)]
pub struct Type
{
	pub id: Rc<str>,
	pub weakness_ids: BoxSlice<Weak<str>>,
	pub resistance_ids: BoxSlice<Weak<str>>,
	pub immunity_ids: BoxSlice<Weak<str>>,
}
impl Identify for Type
{
	fn id(&self) -> &str
	{
		&self.id
	}
}
impl Type
{
	pub fn weakness_to(&self, typ: &Self) -> WeaknessLevel
	{
		if self.immunity_ids.iter().any(|immunity| {
			immunity
				.upgrade()
				.is_some_and(|immunity| &*immunity == typ.id())
		})
		{
			WeaknessLevel::Immunity
		}
		else if self.weakness_ids.iter().any(|weakness| {
			weakness
				.upgrade()
				.is_some_and(|weakness| &*weakness == typ.id())
		})
		{
			WeaknessLevel::Weak
		}
		else if self.resistance_ids.iter().any(|resistance| {
			resistance
				.upgrade()
				.is_some_and(|resistance| &*resistance == typ.id())
		})
		{
			WeaknessLevel::Resist
		}
		else
		{
			WeaknessLevel::Neutral
		}
	}
}

#[derive(Debug)]
pub struct TypePair(Rc<Type>, Option<Rc<Type>>);
impl TypePair
{
	pub fn primary(&self) -> &Type
	{
		&self.0
	}

	pub fn secondary(&self) -> Option<&Type>
	{
		self.1.as_deref()
	}

	pub fn contains(&self, typ: &Type) -> bool
	{
		self.primary().id == typ.id || self.secondary().is_some_and(|t| t.id == typ.id)
	}

	pub fn damage_multiplier_from(&self, typ: &Type) -> f64
	{
		match self.primary().weakness_to(typ)
			+ self
				.secondary()
				.map(|t| t.weakness_to(typ))
				.unwrap_or_default()
		{
			WeaknessLevel::Immunity => 0.0,
			WeaknessLevel::DoubleResist => 0.4,
			WeaknessLevel::Resist => 0.5,
			WeaknessLevel::Neutral => 1.0,
			WeaknessLevel::Weak => 2.0,
			WeaknessLevel::DoubleWeak => 2.5,
		}
	}
}
impl<O> From<(Rc<Type>, O)> for TypePair
where
	O: Into<Option<Rc<Type>>>,
{
	fn from(value: (Rc<Type>, O)) -> Self
	{
		Self(value.0, value.1.into())
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WeaknessLevel
{
	Immunity,
	DoubleResist,
	Resist,
	Neutral,
	Weak,
	DoubleWeak,
}
// this feels more like multipliction. we'll see what we can do about that one -morgan 2023-12-12
impl std::ops::Add<Self> for WeaknessLevel
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output
	{
		if self == Self::Immunity || rhs == Self::Immunity
		{
			Self::Immunity
		}
		else
		{
			(i32::try_from(self).unwrap() + i32::try_from(rhs).unwrap()).into()
		}
	}
}
impl Default for WeaknessLevel
{
	fn default() -> Self
	{
		Self::Neutral
	}
}
impl From<i32> for WeaknessLevel
{
	fn from(value: i32) -> Self
	{
		match value
		{
			..=-2 => Self::DoubleResist,
			-1 => Self::Resist,
			0 => Self::Neutral,
			1 => Self::Weak,
			2.. => Self::DoubleWeak,
		}
	}
}
impl TryFrom<WeaknessLevel> for i32
{
	type Error = ();

	fn try_from(value: WeaknessLevel) -> Result<Self, ()>
	{
		match value
		{
			WeaknessLevel::Immunity => Err(()),
			WeaknessLevel::DoubleResist => Ok(-2),
			WeaknessLevel::Resist => Ok(-1),
			WeaknessLevel::Neutral => Ok(0),
			WeaknessLevel::Weak => Ok(1),
			WeaknessLevel::DoubleWeak => Ok(2),
		}
	}
}
