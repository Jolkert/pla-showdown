#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
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
