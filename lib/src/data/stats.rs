use std::str::FromStr;

pub fn base_action_time(speed: i32) -> i32
{
	match speed
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

#[derive(
	Debug,
	Hash,
	PartialEq,
	Eq,
	Clone,
	Copy,
	serde::Serialize,
	serde::Deserialize,
	strum::Display,
	strum::VariantArray,
)]
#[serde(rename_all = "lowercase")]
pub enum Stat
{
	Hp,
	Atk,
	Def,
	SpAtk,
	SpDef,
	Spe,
}
impl FromStr for Stat
{
	type Err = ParseStatError;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		match s.to_lowercase().as_str()
		{
			"hp" => Ok(Self::Hp),
			"atk" | "attack" => Ok(Self::Atk),
			"def" | "defense" | "defence" => Ok(Self::Def),
			"spa" | "spatk" | "special attack" | "sp attack" => Ok(Self::SpAtk),
			"spd" | "spdef" | "special defense" | "sp defense" | "special defence"
			| "sp defence" => Ok(Self::SpDef),
			"spe" | "speed" => Ok(Self::Spe),
			_ => Err(ParseStatError),
		}
	}
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
impl Nature
{
	pub fn multiplier(self, stat: Stat) -> f64
	{
		match self.effect_on(stat)
		{
			NatureEffect::Decrease => 0.9,
			NatureEffect::Neutral => 1.0,
			NatureEffect::Increase => 1.1,
		}
	}

	pub fn effect_on(self, stat: Stat) -> NatureEffect
	{
		// is there a more idomatic way to do this? maybe! -morgan 2023-12-11
		if self.increased == self.decreased
		{
			NatureEffect::Neutral
		}
		else if stat == self.increased
		{
			NatureEffect::Increase
		}
		else if stat == self.decreased
		{
			NatureEffect::Decrease
		}
		else
		{
			NatureEffect::Neutral
		}
	}
}

impl std::fmt::Display for Nature
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "+{} / -{}", self.increased, self.decreased)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NatureEffect
{
	Decrease,
	Neutral,
	Increase,
}

#[derive(Debug)]
pub struct ParseStatError;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct StatBlock
{
	pub hp: i32,
	pub atk: i32,
	pub def: i32,
	pub spatk: i32,
	pub spdef: i32,
	pub spe: i32,
}
impl std::ops::Index<Stat> for StatBlock
{
	type Output = i32;
	fn index(&self, index: Stat) -> &Self::Output
	{
		match index
		{
			Stat::Hp => &self.hp,
			Stat::Atk => &self.atk,
			Stat::Def => &self.def,
			Stat::SpAtk => &self.spatk,
			Stat::SpDef => &self.spdef,
			Stat::Spe => &self.spe,
		}
	}
}
impl std::ops::IndexMut<Stat> for StatBlock
{
	fn index_mut(&mut self, index: Stat) -> &mut Self::Output
	{
		match index
		{
			Stat::Hp => &mut self.hp,
			Stat::Atk => &mut self.atk,
			Stat::Def => &mut self.def,
			Stat::SpAtk => &mut self.spatk,
			Stat::SpDef => &mut self.spdef,
			Stat::Spe => &mut self.spe,
		}
	}
}
impl StatBlock
{
	pub fn all(val: i32) -> Self
	{
		Self {
			hp: val,
			atk: val,
			def: val,
			spatk: val,
			spdef: val,
			spe: val,
		}
	}

	pub fn map<F>(self, transform: F) -> Self
	where
		F: Fn(i32) -> i32,
	{
		Self {
			hp: transform(self.hp),
			atk: transform(self.atk),
			def: transform(self.def),
			spatk: transform(self.spatk),
			spdef: transform(self.spdef),
			spe: transform(self.spe),
		}
	}

	pub fn generate<F>(generator: F) -> Self
	where
		F: Fn(Stat) -> i32,
	{
		Self {
			hp: generator(Stat::Hp),
			atk: generator(Stat::Atk),
			def: generator(Stat::Def),
			spatk: generator(Stat::SpAtk),
			spdef: generator(Stat::SpDef),
			spe: generator(Stat::Spe),
		}
	}

	// there has to be a better name for this but i cant come up with it -morgan 2023-12-15
	pub fn generate_map<T, F, G>(self, generator: G, transform: F) -> Self
	where
		F: Fn(i32, T) -> i32,
		G: Fn(Stat) -> T,
	{
		Self {
			hp: transform(self.hp, generator(Stat::Hp)),
			atk: transform(self.atk, generator(Stat::Atk)),
			def: transform(self.def, generator(Stat::Def)),
			spatk: transform(self.spatk, generator(Stat::SpAtk)),
			spdef: transform(self.spdef, generator(Stat::SpDef)),
			spe: transform(self.spe, generator(Stat::Spe)),
		}
	}
}

impl std::fmt::Display for StatBlock
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(
			f,
			"{} HP\n{} Atk\n{} Def\n{} SpAtk\n{} SpDef\n{} Spe",
			self.hp, self.atk, self.def, self.spatk, self.spdef, self.spe
		)
	}
}

pub fn effort_bonus(effort_level: i32, pokemon_level: u8, base_stat: i32) -> Option<i32>
{
	// im less convinced with the readability of this one than the stat calc one in ::pokemon
	// consider changing it back -morgan 2023-12-14
	Some(
		(f64::from(base_stat).sqrt().mul_add(
			f64::from(effort_multiplier(effort_level)?),
			f64::from(pokemon_level),
		) / 2.5)
			.round() as i32,
	)
}

fn effort_multiplier(effort_level: i32) -> Option<i32>
{
	// 0, 2, 3, 4, 7, 8, 9, 14, 15, 16, 25
	match effort_level
	{
		0 => Some(0),
		1 => Some(2),
		2 => Some(3),
		3 => Some(4),
		4 => Some(7),
		5 => Some(8),
		6 => Some(9),
		7 => Some(14),
		8 => Some(15),
		9 => Some(16),
		10 => Some(25),
		_ => None,
	}
}
