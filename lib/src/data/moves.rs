use std::{fmt::Write, rc::Rc};

pub use style::*;

use super::{Identify, Side, Type};
use crate::{BoxSlice, BoxStr};

#[derive(Debug, bon::Builder)]
pub struct Move
{
	pub id: BoxStr,
	move_type: Rc<Type>,
	pub category: Category,
	pub pp: u32,
	pub power: StyleTriad,
	pub accuracy: StyleTriad,
	pub user_action_time: StyleTriad,
	pub target_action_time: StyleTriad,
	pub crit_stage: StyleTriad,
	pub effects: BoxSlice<MoveEffect>,
}
impl Identify for Move
{
	fn id(&self) -> &str
	{
		&self.id
	}
}

impl Move
{
	pub fn move_type(&self) -> &Type
	{
		&self.move_type
	}
}

#[derive(
	Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize, strum::Display,
)]
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

		pub fn are_all_equal(&self) -> bool
		{
			self.regular == self.agile && self.regular == self.strong
		}
	}

	impl std::fmt::Display for StyleTriad
	{
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
		{
			if !self.are_all_equal()
			{
				write!(f, "{} | {} | {}", self.regular, self.agile, self.strong)
			}
			else
			{
				write!(f, "{}", self.regular)
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

impl MoveEffect
{
	pub fn condition(&self) -> &MoveEffectCondition
	{
		match self
		{
			Self::Heal { condition, .. } => condition,
			Self::Recoil { condition, .. } => condition,
			Self::ApplyStatus { condition, .. } => condition,
			Self::CureStatus { condition, .. } => condition,
			Self::MultiplyPower { condition, .. } => condition,
			Self::ModifyData { condition, .. } => condition,
			Self::SwapOffenseAndDefense { condition, .. } => condition,
		}
	}
}

impl std::fmt::Display for MoveEffect
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::Heal {
				percent_of,
				percent,
				..
			} =>
			{
				write!(f, "Heals {percent} percent of {percent_of}")?;
			}
			Self::Recoil {
				percent_of,
				percent,
				..
			} =>
			{
				write!(
					f,
					"User takes {percent} percent of {percent_of} as recoil damage"
				)?;
			}
			Self::ApplyStatus {
				to,
				status_option_ids,
				duration,
				chance,
				..
			} =>
			{
				if chance.are_all_equal() && chance.regular == 100
				{
					write!(f, "Inflicts ")?;
				}
				else
				{
					write!(f, "{chance} percent chance to inflict ")?;
				}

				write!(f, "{to} with ")?;
				write_or_string(f, status_option_ids)?;
				write!(f, " for {duration} turns")?;
			}
			Self::CureStatus { of, status_ids, .. } =>
			{
				write!(f, "Cures {of} of ")?;
				write_or_string(f, status_ids)?;
			}
			Self::MultiplyPower { multiplier, .. } =>
			{
				write!(f, "{multiplier}x power")?;
			}
			Self::SwapOffenseAndDefense { of, .. } =>
			{
				write!(f, "Swaps {of}'s offensive and defensive stats")?;
			}
			Self::ModifyData {
				power,
				accuracy,
				user_action_time,
				target_action_time,
				crit_stage,
				..
			} =>
			{
				write!(f, "Move stats become: ")?;
				let mut data_string = String::new();
				if let Some(power) = power
				{
					write!(data_string, "{power} power, ")?;
				}
				if let Some(accuracy) = accuracy
				{
					write!(data_string, "{accuracy} accuracy, ")?;
				}
				if let Some(user_action_time) = user_action_time
				{
					write!(data_string, "{user_action_time} AT (user), ")?;
				}
				if let Some(target_action_time) = target_action_time
				{
					write!(data_string, "{target_action_time} AT (target), ")?;
				}
				if let Some(crit_stage) = crit_stage
				{
					write!(data_string, "{crit_stage} crit, ")?;
				}

				_ = data_string.pop();
				_ = data_string.pop();
				write!(f, "{data_string}")?;
			}
		}

		if !self.condition().both_are_none()
		{
			write!(f, " if {}", self.condition())?;
		}

		Ok(())
	}
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

impl std::fmt::Display for MoveEffectCondition
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		if let Some(user_condition) = &self.user
		{
			write!(f, "user {user_condition}")?;
			if self.target.is_some()
			{
				write!(f, " and ")?;
			}
		}
		if let Some(target_condition) = &self.target
		{
			write!(f, "target {target_condition}")?;
		}

		Ok(())
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

impl std::fmt::Display for PokemonConditionData
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		if let Some(species) = &self.species_id
		{
			write!(f, "is {species}")?;
			if self.status_ids.is_some()
			{
				write!(f, " and ")?;
			}
		}
		if let Some(statuses) = &self.status_ids
		{
			write!(f, "is afflicted with ")?;
			write_or_string(f, statuses)?;
		}

		Ok(())
	}
}

fn write_or_string<T: std::fmt::Display>(
	f: &mut std::fmt::Formatter<'_>,
	options: &[T],
) -> std::fmt::Result
{
	if options.len() == 1
	{
		write!(f, "{}", options[0])?;
	}
	else
	{
		for (i, option) in options.iter().enumerate()
		{
			if i == 0
			{
				write!(f, "{option}")?;
			}
			else if i == options.len() - 1
			{
				write!(f, ", or {option}")?;
			}
			else
			{
				write!(f, ", {option}")?;
			}
		}
	}

	Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DamageOrMaxHp
{
	DamageDealt,
	MaxHp,
}
impl std::fmt::Display for DamageOrMaxHp
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			Self::DamageDealt => write!(f, "damage dealt"),
			Self::MaxHp => write!(f, "max HP"),
		}
	}
}
