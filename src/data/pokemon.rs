use super::{Move, Stat};
use crate::data::{StatBlock, TypePair};
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
	fn new(species: &'a Species<'a>) -> Self
	{
		Self {
			species,
			nickname: None,
			is_shiny: false,
			level: 100,
			nature: Nature::default(),
			effort_levels: StatBlock {
				hp: 0,
				atk: 0,
				def: 0,
				spatk: 0,
				spdef: 0,
				spe: 0,
			},
			moveset: HashSet::new(),
		}
	}

	fn name(&self) -> &str
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

	fn stats() -> StatBlock
	{
		todo!()
	}

	fn nickname(mut self, nickname: Option<String>) -> Self
	{
		self.nickname = nickname;
		self
	}
	fn set_shiny(mut self, is_shiny: bool) -> Self
	{
		self.is_shiny = is_shiny;
		self
	}
	fn level(mut self, level: u8) -> Self
	{
		self.level = level;
		self
	}
	fn nature(mut self, nature: Nature) -> Self
	{
		self.nature = nature;
		self
	}
	fn effort_levels(mut self, effort_levels: StatBlock) -> Self
	{
		self.effort_levels = effort_levels;
		self
	}
	fn add_move(mut self, mv: &'a Move<'a>) -> Self
	{
		self.moveset.insert(mv);
		self
	}
	fn add_moves<I>(mut self, moves: I) -> Self
	where
		I: IntoIterator<Item = &'a Move<'a>>,
	{
		moves.into_iter().for_each(|m| {
			self.moveset.insert(m);
		});
		self
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Nature
{
	pub increased: Stat,
	pub decreased: Stat,
}
impl Default for Nature
{
	fn default() -> Self
	{
		Self {
			increased: Stat::Spe,
			decreased: Stat::Spe,
		}
	}
}
