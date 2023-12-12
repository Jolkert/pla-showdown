use crate::data;

use data::{Effect, Move, Nature, Stat, StatBlock, StatusCondition, Type, TypePair};
use std::{collections::HashSet, slice::Iter};

#[derive(Debug)]
pub struct Species<'a>
{
	pub id: Box<str>,
	pub base_stats: StatBlock,
	pub types: TypePair<'a>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side
{
	User,
	Target,
}

#[derive(Debug)]
pub struct Pokemon<'a>
{
	pub species: &'a Species<'a>,
	pub nickname: Option<String>,
	pub is_shiny: bool,
	pub level: u8,
	pub nature: Nature,
	pub effort_levels: StatBlock,
	pub moveset: HashSet<&'a Move<'a>>,
}
impl<'a> Pokemon<'a>
{
	pub fn new(species: &'a Species<'a>) -> Self
	{
		Self {
			species,
			nickname: None,
			is_shiny: false,
			level: 100,
			nature: Nature::default(),
			effort_levels: StatBlock::all(10),
			moveset: HashSet::new(),
		}
	}

	pub fn name(&self) -> &str
	{
		if let Some(nickname) = &self.nickname
		{
			nickname
		}
		else
		{
			&self.species.id
		}
	}

	pub fn stats(&self) -> StatBlock
	{
		StatBlock::for_each_stat(|stat| self.calculate_stat(stat))
	}

	fn calculate_stat(&self, stat: Stat) -> i32
	{
		let base = self.species.base_stats[stat];
		if stat == Stat::Hp
		{
			((self.level as f32 / 100.0 + 1.0) * base as f32 + self.level as f32).floor() as i32
				+ data::effort_bonus(self.effort_levels[stat], self.level, base)
					.expect("effort level was not in range [0, 10]")
		}
		else
		{
			(((self.level as f32 / 50.0 + 1.0) * base as f32 / 1.5) * self.nature.multiplier(stat))
				.floor() as i32 + data::effort_bonus(self.effort_levels[stat], self.level, base)
				.expect("effort level was not in range [0, 10]")
		}
	}

	pub fn nickname(mut self, nickname: Option<String>) -> Self
	{
		self.nickname = nickname;
		self
	}
	pub fn set_shiny(mut self, is_shiny: bool) -> Self
	{
		self.is_shiny = is_shiny;
		self
	}
	pub fn level(mut self, level: u8) -> Self
	{
		self.level = level;
		self
	}
	pub fn nature(mut self, nature: Nature) -> Self
	{
		self.nature = nature;
		self
	}
	pub fn effort_levels(mut self, effort_levels: StatBlock) -> Self
	{
		self.effort_levels = effort_levels;
		self
	}
	pub fn add_move(mut self, mv: &'a Move<'a>) -> Self
	{
		self.moveset.insert(mv);
		self
	}
	pub fn add_moves<I>(mut self, moves: I) -> Self
	where
		I: IntoIterator<Item = &'a Move<'a>>,
	{
		for mv in moves.into_iter()
		{
			self.moveset.insert(mv);
		}

		self
	}
}

pub struct BattlePokemon<'a>
{
	pub pokemon: &'a Pokemon<'a>,
	pub damage: i32,
	pub action_time: i32,
	pub non_volatile_status: Option<AppliedStatus<'a>>,
	pub volatile_statuses: Vec<AppliedStatus<'a>>,
}
impl<'a> BattlePokemon<'a>
{
	pub fn status_effects(&self) -> impl Iterator<Item = &Effect>
	{
		let non_volatile = if let Some(condition) = &self.non_volatile_status
		{
			condition.effects()
		}
		else
		{
			[].iter()
		};

		let volatile = self
			.volatile_statuses
			.iter()
			.flat_map(|condition| condition.effects());

		non_volatile.chain(volatile)
	}

	pub fn effective_stats(&self) -> StatBlock
	{
		self.pokemon.stats().map_all(
			|st| self.multiplier_to_stat(st),
			|init, mult| (init as f32 * mult) as i32,
		)
	}

	pub fn multiplier_to_stat(&self, st: Stat) -> f32
	{
		self.status_effects()
			.filter_map(|eff| {
				if let Effect::ModifyStat { stat,multiplier,} = eff && *stat == st
				{
					Some(multiplier)
				}
				else
				{
					None
				}
			})
			.product()
	}

	pub fn is_type(&self, typ: &Type) -> bool
	{
		self.pokemon.species.types.contains(typ)
	}

	pub fn base_action_time(&self) -> i32
	{
		match self.effective_stats().spe
		{
			..=15 => 14,
			16..=31 => 13,
			32..=55 => 12,
			56..=88 => 11,
			89..=129 => 10,
			130..=181 => 9,
			182..=242 => 8,
			243..=316 => 7,
			317..=401 => 6,
			402.. => 5,
		}
	}
}

pub struct AppliedStatus<'a>
{
	pub condition: &'a StatusCondition<'a>,
	pub duration: i32,
	pub move_source: &'a Move<'a>,
}
impl<'a> AppliedStatus<'a>
{
	pub fn tick_down(&mut self)
	{
		self.duration -= 1;
	}

	pub fn effects(&self) -> Iter<Effect>
	{
		self.condition.effects.iter()
	}
}
