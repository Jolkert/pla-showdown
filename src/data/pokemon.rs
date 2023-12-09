use crate::data::{StatBlock, TypePair};

#[derive(Debug)]
pub struct Species<'a>
{
	pub id: Box<str>,
	pub base_stats: StatBlock,
	pub types: TypePair<'a>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side
{
	User,
	Target,
}
