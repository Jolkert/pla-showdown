use super::{Identify, Side, Type};
use crate::{BoxSlice, BoxStr};

pub use style::*;

#[derive(Debug)]
pub struct Move<'a>
{
	pub id: BoxStr,
	pub move_type: &'a Type,
	pub category: Category,
	pub pp: u32,
	pub power: StyleTriad,
	pub accuracy: StyleTriad,
	pub user_action_time: StyleTriad,
	pub target_action_time: StyleTriad,
	pub crit_stage: StyleTriad,
	pub effects: BoxSlice<MoveEffect>,
}
impl<'a> Identify for Move<'a>
{
	fn id(&self) -> &str
	{
		&self.id
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category
{
	Physical,
	Special,
	Status,
	All,
}
impl Category
{
	pub fn is_damaging(self) -> bool
	{
		matches!(self, Self::Physical | Self::Special)
	}
}

mod style
{
	#[derive(Debug, Clone, Copy)]
	pub enum Style
	{
		Regular,
		Agile,
		Strong,
	}

	#[derive(Debug, Hash, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
	pub struct StyleTriad
	{
		pub regular: i32,
		pub agile: i32,
		pub strong: i32,
	}
	impl std::ops::Index<Style> for StyleTriad
	{
		type Output = i32;
		fn index(&self, index: Style) -> &Self::Output
		{
			match index
			{
				Style::Regular => &self.regular,
				Style::Agile => &self.agile,
				Style::Strong => &self.strong,
			}
		}
	}
	impl From<i32> for StyleTriad
	{
		fn from(value: i32) -> Self
		{
			Self::all(value)
		}
	}
	impl StyleTriad
	{
		pub fn new(regular: i32, agile: i32, strong: i32) -> Self
		{
			Self {
				regular,
				agile,
				strong,
			}
		}

		pub fn all(val: i32) -> Self
		{
			Self {
				regular: val,
				agile: val,
				strong: val,
			}
		}
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "effect_type")]
pub enum MoveEffect
{
	Heal
	{
		percent_of: DamageOrMaxHp,
		percent: StyleTriad,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
	Recoil
	{
		percent_of: DamageOrMaxHp,
		percent: StyleTriad,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
	ApplyStatus
	{
		to: Side,
		#[serde(rename = "status_options")]
		status_option_ids: BoxSlice<BoxStr>,
		duration: StyleTriad,
		#[serde(default = "always")]
		chance: StyleTriad,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
	CureStatus
	{
		of: Side,
		#[serde(rename = "statuses")]
		status_ids: BoxSlice<BoxStr>,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
	MultiplyPower
	{
		multiplier: i32,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
	ModifyData
	{
		// TODO: god this hurts my eyes we gotta fix this somehow -morgan 2023-12-08
		#[serde(skip_serializing_if = "Option::is_none")]
		power: Option<StyleTriad>,
		#[serde(skip_serializing_if = "Option::is_none")]
		accuracy: Option<StyleTriad>,
		#[serde(skip_serializing_if = "Option::is_none")]
		user_action_time: Option<StyleTriad>,
		#[serde(skip_serializing_if = "Option::is_none")]
		target_action_time: Option<StyleTriad>,
		#[serde(skip_serializing_if = "Option::is_none")]
		crit_stage: Option<StyleTriad>,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
	SwapOffenseAndDefense
	{
		of: Side,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
}

fn always() -> StyleTriad
{
	StyleTriad::all(100)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MoveEffectCondition
{
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<PokemonConditionData>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub target: Option<PokemonConditionData>,
}
impl MoveEffectCondition
{
	pub fn both_are_none(&self) -> bool
	{
		self.user.is_none() && self.target.is_none()
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonConditionData
{
	#[serde(rename = "species", skip_serializing_if = "Option::is_none")]
	pub species_id: Option<BoxStr>,
	#[serde(rename = "status", skip_serializing_if = "Option::is_none")]
	pub status_ids: Option<BoxSlice<BoxStr>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DamageOrMaxHp
{
	DamageDealt,
	MaxHp,
}
