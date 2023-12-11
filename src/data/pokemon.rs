use crate::data;

use data::{Move, Nature, Stat, StatBlock, TypePair};
use std::collections::HashSet;

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
			effort_levels: StatBlock {
				hp: 10,
				atk: 10,
				def: 10,
				spatk: 10,
				spdef: 10,
				spe: 10,
			},
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
		StatBlock {
			hp: self.calculate_stat(Stat::Hp),
			atk: self.calculate_stat(Stat::Atk),
			def: self.calculate_stat(Stat::Def),
			spatk: self.calculate_stat(Stat::SpAtk),
			spdef: self.calculate_stat(Stat::SpDef),
			spe: self.calculate_stat(Stat::Spe),
		}
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
