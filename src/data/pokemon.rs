use crate::data;

use data::{
	AppliedStatus, Category, Effect, Move, Nature, Stat, StatBlock, StatusCondition, Style,
	StyleTriad, Type, TypePair, Volatility,
};
use rand::Rng;
use std::collections::{HashMap, HashSet};

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
			((f64::from(self.level) / 100.0 + 1.0) * f64::from(base) + f64::from(self.level))
				.floor() as i32 + data::effort_bonus(self.effort_levels[stat], self.level, base)
				.expect("effort level was not in range [0, 10]")
		}
		else
		{
			(((f64::from(self.level) / 50.0 + 1.0) * f64::from(base) / 1.5)
				* self.nature.multiplier(stat))
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
		for mv in moves
		{
			self.moveset.insert(mv);
		}

		self
	}

	pub fn base_action_time(&self) -> i32
	{
		data::base_action_time(self.stats().spe)
	}
}

pub struct BattlePokemon<'a>
{
	pub pokemon: &'a Pokemon<'a>,
	damage: i32,
	action_time: i32,
	non_volatile_status: Option<AppliedStatus<'a>>,
	volatile_statuses: HashMap<Box<str>, AppliedStatus<'a>>,
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
			.filter_map(Option::as_ref)
			.chain(self.volatile_statuses.iter().map(|it| it.1))
	}

	pub fn status_effects(&self) -> impl Iterator<Item = &Effect>
	{
		self.status_conditions().flat_map(AppliedStatus::effects)
	}

	pub fn effective_stats(&self) -> StatBlock
	{
		self.pokemon.stats().map_all(
			|st| self.multiplier_to_stat(st),
			|init, mult| (f64::from(init) * mult) as i32,
		)
	}

	pub fn apply_status(
		&mut self,
		condition: &'a StatusCondition,
		duration: i32,
		source_move: &'a Move,
	)
	{
		if !condition.immune_types.iter().any(|it| self.is_type(it))
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
			.iter_mut()
			.for_each(|it| it.1.tick_down());
		self.volatile_statuses.retain(|_, it| it.duration > 0);
	}

	pub fn multiplier_to_stat(&self, st: Stat) -> f64
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
		let crit_stages = mv.crit_stage[style]
			+ attacker
				.status_effects()
				.map(|it| {
					if let Effect::ModifyCritChance { stages } = it
					{
						*stages
					}
					else
					{
						1
					}
				})
				.sum::<i32>();

		let crit_chance = match crit_stages
		{
			..=0 => 24,
			1 => 8,
			2 => 2,
			3.. => 1,
		};

		let crit_multiplier: f64 = if rand::thread_rng().gen_range(0..crit_chance) == 0
		{
			1.5
		}
		else
		{
			1.0
		};

		(f64::from(
			Self::calculate_damage_no_roll(
				attacker,
				target,
				&mv.power,
				mv.category,
				mv.move_type,
				style,
			) * rand::thread_rng().gen_range(85..100)
				/ 100,
		) * crit_multiplier) as i32
	}

	pub fn calculate_damage_no_roll(
		attacker: &BattlePokemon,
		target: &BattlePokemon,
		base_power: &StyleTriad<i32>,
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
		let stab_multiplier: f64 = if attacker.is_type(move_type)
		{
			1.25
		}
		else
		{
			1.0
		};

		let effects_multiplier: f64 = attacker
			.status_effects()
			.map(|it| (Side::User, it))
			.chain(target.status_effects().map(|it| (Side::Target, it)))
			.map(|eff| {
				if let Effect::DamageMultiplier { side, multiplier, move_category } = eff.1
					&& *side == eff.0
					&& (*move_category == Category::All || *move_category == category)
				{
					*multiplier
				}
				else
				{
					1.0
				}
			})
			.product();

		(f64::from(base_damage) * effects_multiplier * type_multiplier * stab_multiplier) as i32
	}
}
