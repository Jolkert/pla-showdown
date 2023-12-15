use crate::data;

use data::{Category, Move, Side, Stat, Type};

#[derive(Debug)]
pub struct StatusCondition<'a>
{
	pub id: Box<str>,
	pub volatility: Volatility,
	pub immune_types: Box<[&'a Type]>,
	pub effects: Box<[Effect]>,
}

pub struct AppliedStatus<'a>
{
	pub condition: &'a StatusCondition<'a>,
	pub duration: i32,
	pub source_move: &'a Move<'a>,
}
impl<'a> AppliedStatus<'a>
{
	pub fn tick_down(&mut self)
	{
		self.duration -= 1;
	}

	pub fn effects(&self) -> impl Iterator<Item = &Effect>
	{
		self.condition.effects.iter()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Volatility
{
	Volatile,
	NonVolatile,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "effect_type")]
pub enum Effect
{
	ModifyStat
	{
		stat: Stat, multiplier: f64
	},
	DamageMultiplier
	{
		side: Side,
		multiplier: f64,
		#[serde(default = "category_all", skip_serializing_if = "category_is_all")]
		move_category: Category,
	},
	CancelTurn
	{
		chance: i32
	},
	TurnEndDamageFraction
	{
		fraction_denominator: i32
	},
	TurnEndDamageMove
	{
		base_power: i32
	},
	EvasionModifier
	{
		multiplier: f64
	},
	ModifyCritChance
	{
		stages: i32
	},
	SwapStats
	{
		stats: (Stat, Stat)
	},
}
impl Effect
{
	pub fn damge_multiplier(&self, category: Category, side: Side) -> f64
	{
		if let Self::DamageMultiplier { side: sd, multiplier, move_category } = self
			&& *sd == side
			&& (*move_category == Category::All || *move_category == category)
		{
			*multiplier
		}
		else
		{
			1.0
		}
	}

	pub fn crit_bonus(&self) -> i32
	{
		if let Self::ModifyCritChance { stages } = self
		{
			*stages
		}
		else
		{
			0
		}
	}
}

// serde default should let you supply a unit enum variant and the fact that it doesnt makes me angry
// -morgan 2023-12-09
fn category_all() -> Category
{
	Category::All
}

// serde demands a reference clippy shush -morgan 2023-12-12
#[allow(clippy::trivially_copy_pass_by_ref)]
fn category_is_all(cat: &Category) -> bool
{
	*cat == Category::All
}
