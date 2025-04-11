use std::collections::HashMap;

use crate::{
	BoxSlice, BoxStr,
	data::{Effect, Identify, StatusCondition, Type, Volatility},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerStatus
{
	pub id: BoxStr,
	pub volatility: Volatility,
	#[serde(
		rename = "immune_types",
		default = "super::empty_slice",
		skip_serializing_if = "<[_]>::is_empty"
	)]
	pub immune_type_ids: BoxSlice<BoxStr>,
	pub effects: BoxSlice<Effect>,
}
impl SerStatus
{
	pub fn into_status(self, type_map: &HashMap<BoxStr, Type>) -> StatusCondition
	{
		StatusCondition {
			id: self.id,
			volatility: self.volatility,
			effects: self.effects,
			immune_types: self
				.immune_type_ids
				.iter()
				.map(|it| type_map.get(it).unwrap())
				.collect(),
		}
	}
}
impl Identify for SerStatus
{
	fn id(&self) -> BoxStr
	{
		self.id.clone()
	}
}
