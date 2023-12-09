use super::{Category, Side, Stat, Type};

#[derive(Debug)]
pub struct StatusCondition<'a>
{
	pub id: Box<str>,
	pub volatility: Volatility,
	pub immune_types: Box<[&'a Type]>,
	pub effects: Box<[Effect]>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
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
		stat: Stat,
		multiplier: f32,
	},
	DamageMultiplier
	{
		side: Side,
		multiplier: f32,
		move_category: Category,
	},
	CancelTurn
	{
		chance: i32,
	},
	TurnEndDamageFraction
	{
		fraction_denominator: i32,
	},
	TurnEndDamageMove
	{
		base_power: i32,
	},
	EvasionModifier(f32),
}
