use crate::data::Type;

pub use style::*;

use super::Side;

#[derive(Debug)]
pub struct Move<'a>
{
	pub id: Box<str>,
	pub move_type: &'a Type,
	pub category: Category,
	pub pp: u32,
	pub power: StyleTriad<i32>,
	pub accuracy: StyleTriad<i32>,
	pub user_action_time: StyleTriad<i32>,
	pub target_action_time: StyleTriad<i32>,
	pub crit_stage: StyleTriad<i32>,
	pub effects: Box<[MoveEffect]>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category
{
	Physical,
	Special,
	Status,
}
impl Category
{
	pub fn is_damaging(&self) -> bool
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

	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
	pub struct StyleTriad<T>
	where
		T: Copy,
	{
		pub regular: T,
		pub agile: T,
		pub strong: T,
	}
	impl<T> std::ops::Index<Style> for StyleTriad<T>
	where
		T: Copy,
	{
		type Output = T;
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
	impl<T> StyleTriad<T>
	where
		T: Copy,
	{
		pub fn new(regular: T, agile: T, strong: T) -> Self
		{
			Self {
				regular,
				agile,
				strong,
			}
		}

		pub fn all(val: T) -> Self
		{
			StyleTriad {
				regular: val,
				agile: val,
				strong: val,
			}
		}
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "effect_type")]
pub enum MoveEffect
{
	Heal
	{
		percent_of: DamageOrMaxHp,
		percent: StyleTriad<i32>,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
	Recoil
	{
		percent_of: DamageOrMaxHp,
		percent: StyleTriad<i32>,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
	ApplyStatus
	{
		to: Side,
		#[serde(rename = "status_options")]
		status_option_ids: Box<[Box<str>]>,
		duration: StyleTriad<i32>,
		chance: StyleTriad<i32>,
		#[serde(default, skip_serializing_if = "MoveEffectCondition::both_are_none")]
		condition: MoveEffectCondition,
	},
	CureStatus
	{
		of: Side,
		#[serde(rename = "statuses")]
		status_ids: Box<[Box<str>]>,
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
		power: Option<StyleTriad<i32>>,
		#[serde(skip_serializing_if = "Option::is_none")]
		accuracy: Option<StyleTriad<i32>>,
		#[serde(skip_serializing_if = "Option::is_none")]
		user_action_time: Option<StyleTriad<i32>>,
		#[serde(skip_serializing_if = "Option::is_none")]
		target_action_time: Option<StyleTriad<i32>>,
		#[serde(skip_serializing_if = "Option::is_none")]
		crit_stage: Option<StyleTriad<i32>>,
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonConditionData
{
	#[serde(rename = "species", skip_serializing_if = "Option::is_none")]
	pub species_id: Option<Box<str>>,
	#[serde(rename = "status", skip_serializing_if = "Option::is_none")]
	pub status_ids: Option<Box<[Box<str>]>>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DamageOrMaxHp
{
	DamageDealt,
	MaxHp,
}
