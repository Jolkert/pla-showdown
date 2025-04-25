use std::{collections::HashMap, rc::Rc};

use rand::Rng;

use super::{
	AppliedStatus, Category, Effect, IdSet, Identifiable, Identify, Move, Nature, Stat, StatBlock,
	StatusCondition, Style, StyleTriad, Type, TypePair, Volatility,
};
use crate::{BoxStr, data};

#[derive(Debug)]
pub struct Species
{
	pub id: BoxStr,
	pub base_stats: StatBlock,
	pub types: TypePair,
}
impl Identify for Species
{
	fn id(&self) -> &str
	{
		&self.id
	}
}

#[derive(
	Debug, Hash, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize, strum::Display,
)]
#[serde(rename_all = "snake_case")]
pub enum Side
{
	User,
	Target,
}

#[derive(Debug)]
pub struct Pokemon
{
	pub species: Rc<Species>,
	pub nickname: Option<String>,
	pub is_shiny: bool,
	pub level: u8,
	pub nature: Nature,
	pub effort_levels: StatBlock,
	pub moveset: IdSet<Rc<Move>>,
}
impl Pokemon
{
	pub fn new(species: &Rc<Species>) -> Self
	{
		Self {
			species: species.clone(),
			nickname: None,
			is_shiny: false,
			level: 100,
			nature: Nature::default(),
			effort_levels: StatBlock::all(10),
			moveset: IdSet::new(),
		}
	}

	pub fn with_nickname(species: &Rc<Species>, nickname: Option<String>) -> Self
	{
		let mut pkmn = Self::new(species);
		pkmn.nickname = nickname;
		pkmn
	}

	pub fn name(&self) -> &str
	{
		self.nickname.as_deref().unwrap_or(&self.species.id)
	}

	pub fn stats(&self) -> StatBlock
	{
		StatBlock::generate(|stat| self.calculate_stat(stat))
	}

	fn calculate_stat(&self, stat: Stat) -> i32
	{
		let base = self.species.base_stats[stat];
		if stat == Stat::Hp
		{
			// is this more readable? i think maybe ¯\_(ツ)_/¯
			// the lack of a giant block of parentheses is at least nice -morgan 2023-12-14
			f64::floor(
				f64::from(self.level)
					.mul_add(0.01, 1.0)
					.mul_add(f64::from(base), f64::from(self.level))
					.floor(),
			) as i32 + data::effort_bonus(self.effort_levels[stat], self.level, base)
				.expect("effort level was not in range [0, 10]")
		}
		else
		{
			f64::floor(
				(f64::from(self.level).mul_add(0.02, 1.0) * f64::from(base) / 1.5).floor()
					* self.nature.multiplier(stat),
			) as i32 + data::effort_bonus(self.effort_levels[stat], self.level, base)
				.expect("effort level was not in range [0, 10]")
		}
	}

	pub fn set_nickname(&mut self, nickname: impl Into<Option<String>>)
	{
		self.nickname = nickname.into();
	}
	pub fn set_shiny(&mut self, is_shiny: bool)
	{
		self.is_shiny = is_shiny;
	}
	pub fn set_level(&mut self, level: u8)
	{
		self.level = level;
	}
	pub fn set_nature(&mut self, nature: Nature)
	{
		self.nature = nature;
	}
	pub fn set_effort_levels(&mut self, effort_levels: StatBlock)
	{
		self.effort_levels = effort_levels;
	}
	pub fn add_move(&mut self, mv: &Rc<Move>)
	{
		self.moveset.insert(mv.clone());
	}
	pub fn add_moves<I>(&mut self, moves: I)
	where
		I: IntoIterator<Item = Identifiable<Rc<Move>>>,
	{
		for mv in moves
		{
			self.moveset.insert(mv.as_ref().clone());
		}
	}

	pub fn base_action_time(&self) -> i32
	{
		data::base_action_time(self.stats().spe)
	}
}

pub struct BattlePokemon<'a>
{
	pokemon: &'a Pokemon,
	damage: i32,
	action_time: i32,
	non_volatile_status: Option<AppliedStatus<'a>>,
	volatile_statuses: HashMap<BoxStr, AppliedStatus<'a>>,
}
impl<'a> BattlePokemon<'a>
{
	pub fn new(pokemon: &'a Pokemon) -> Self
	{
		Self {
			pokemon,
			damage: 0,
			action_time: pokemon.base_action_time(),
			non_volatile_status: None,
			volatile_statuses: HashMap::new(),
		}
	}

	pub fn current_hp(&self) -> i32
	{
		self.effective_stats().hp - self.damage
	}

	pub fn status_conditions(&self) -> impl Iterator<Item = &AppliedStatus>
	{
		std::iter::once(&self.non_volatile_status)
			.flatten()
			.chain(self.volatile_statuses.values())
	}

	pub fn status_effects(&self) -> impl Iterator<Item = &Effect>
	{
		self.status_conditions().flat_map(AppliedStatus::effects)
	}

	pub fn effective_stats(&self) -> StatBlock
	{
		self.pokemon.stats().generate_map(
			|st| self.multiplier_to_stat(st),
			|current, mult| (f64::from(current) * mult).trunc() as i32,
		)
	}

	pub fn apply_status(
		&mut self,
		condition: &'a StatusCondition,
		duration: i32,
		source_move: &'a Move,
	)
	{
		if !condition.immune_types().any(|typ| self.is_type(typ))
		{
			let applied_status = AppliedStatus {
				condition,
				duration,
				source_move,
			};
			match condition.volatility
			{
				Volatility::NonVolatile => self.non_volatile_status = Some(applied_status),
				Volatility::Volatile =>
				{
					self.volatile_statuses
						.insert(applied_status.condition.id.clone(), applied_status);
				}
			}
		}
	}

	pub fn tick_statuses(&mut self)
	{
		if let Some(status) = &mut self.non_volatile_status
		{
			status.tick_down();
			if status.duration == 0
			{
				self.non_volatile_status = None;
			}
		}
		self.volatile_statuses
			.values_mut()
			.for_each(|status| status.tick_down());
		self.volatile_statuses
			.retain(|_, status| status.duration > 0);
	}

	pub fn multiplier_to_stat(&self, st: Stat) -> f64
	{
		self.status_effects()
			.filter_map(|eff| {
				if let Effect::ModifyStat { stat, multiplier } = eff
					&& *stat == st
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

	pub fn types(&self) -> &TypePair
	{
		&self.pokemon.species.types
	}
	pub fn is_type(&self, typ: &Type) -> bool
	{
		self.types().contains(typ)
	}

	pub fn base_action_time(&self) -> i32
	{
		data::base_action_time(self.effective_stats().spe)
	}

	pub fn calculate_damage(
		attacker: &BattlePokemon,
		target: &BattlePokemon,
		mv: &Move,
		style: Style,
	) -> i32
	{
		let base_damage = Self::calculate_damage_no_roll(
			attacker,
			target,
			&mv.power,
			mv.category,
			mv.move_type(),
			style,
		);

		let crit_chance = match mv.crit_stage[style]
			+ attacker
				.status_effects()
				.map(Effect::crit_bonus)
				.sum::<i32>()
		{
			..=0 => 24,
			1 => 8,
			2 => 2,
			3.. => 1,
		};

		let crit_multiplier: f64 = if rand::rng().random_range(0..crit_chance) == 0
		{
			1.5
		}
		else
		{
			1.0
		};

		(f64::from(base_damage * rand::rng().random_range(85..100) / 100) * crit_multiplier).floor()
			as i32
	}

	pub fn calculate_damage_no_roll(
		attacker: &BattlePokemon,
		target: &BattlePokemon,
		base_power: &StyleTriad,
		category: Category,
		move_type: &Type,
		style: Style,
	) -> i32
	{
		let attack_stat = attacker.effective_stats()[if category == Category::Physical
		{
			Stat::Atk
		}
		else
		{
			Stat::SpAtk
		}];

		let defense_stat = target.effective_stats()[if category == Category::Physical
		{
			Stat::Def
		}
		else
		{
			Stat::SpDef
		}];

		let base_damage = (((100 + attack_stat + (15 * i32::from(attacker.pokemon.level)))
			* base_power[style])
			/ (defense_stat + 50))
			/ 5;

		let type_multiplier = target.types().damage_multiplier_from(move_type);
		let stab_multiplier = if attacker.is_type(move_type)
		{
			1.25
		}
		else
		{
			1.0
		};

		let effects_multiplier = attacker
			.status_effects()
			.map(|effect| (Side::User, effect))
			.chain(target.status_effects().map(|effect| (Side::Target, effect)))
			.map(|(side, effect)| effect.damge_multiplier(category, side))
			.product::<f64>();

		(f64::from(base_damage) * effects_multiplier * type_multiplier * stab_multiplier) as i32
	}
}

impl<'a> std::ops::Deref for BattlePokemon<'a>
{
	type Target = Pokemon;

	fn deref(&self) -> &Self::Target
	{
		self.pokemon
	}
}
