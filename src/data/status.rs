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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Effect
{
	StatModifier
	{
		stat: Stat,
		multiplier: f32,
	},
	DamageMultiplier
	{
		side: Side,
		multiplier: f32,
		category: Category,
	},
	CancelTurn
	{
		chance: i32,
	},
	TurnEndDamageFraction(i32),
	TurnEndDamageMove
	{
		base_power: i32,
	},
	EvasionModifier(f32),
}
