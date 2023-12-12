use crate::data;

use data::Identifiable;

#[derive(Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Type
{
	pub id: Box<str>,
	#[serde(rename = "weaknesses")]
	pub weakness_ids: Box<[Box<str>]>,
	#[serde(rename = "resistances")]
	pub resistance_ids: Box<[Box<str>]>,
	#[serde(rename = "immunities")]
	pub immunity_ids: Box<[Box<str>]>,
}
impl Identifiable for Type
{
	fn id(&self) -> Box<str>
	{
		self.id.clone()
	}
}
impl Type
{
	pub fn weakness_to(&self, typ: &Type) -> WeaknessLevel
	{
		if self.immunity_ids.contains(&typ.id)
		{
			WeaknessLevel::Immunity
		}
		else if self.weakness_ids.contains(&typ.id)
		{
			WeaknessLevel::Weak
		}
		else if self.resistance_ids.contains(&typ.id)
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
pub struct TypePair<'a>(pub &'a Type, pub Option<&'a Type>);
impl<'a> TypePair<'a>
{
	pub fn contains(&self, typ: &Type) -> bool
	{
		self.0.id == typ.id || self.1.is_some_and(|t| t.id == typ.id)
	}

	pub fn damage_multiplier_from(&self, typ: &Type) -> f32
	{
		match self.0.weakness_to(typ) + self.1.map(|t| t.weakness_to(typ)).unwrap_or_default()
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
			2.. => Self::DoubleWeak,
			1 => Self::Weak,
			0 => Self::Neutral,
			-1 => Self::Resist,
			..=-2 => Self::DoubleResist,
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
			WeaknessLevel::DoubleWeak => Ok(2),
			WeaknessLevel::Weak => Ok(1),
			WeaknessLevel::Neutral => Ok(0),
			WeaknessLevel::Resist => Ok(-1),
			WeaknessLevel::DoubleResist => Ok(-1),
			WeaknessLevel::Immunity => Err(()),
		}
	}
}
