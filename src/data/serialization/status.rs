use std::collections::HashMap;

use crate::data::{Effect, Identifiable, StatusCondition, Type, Volatility};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerStatus
{
	pub id: Box<str>,
	pub volatility: Volatility,
	#[serde(
		rename = "immune_types",
		default = "super::empty_slice",
		skip_serializing_if = "<[_]>::is_empty"
	)]
	pub immune_type_ids: Box<[Box<str>]>,
	pub effects: Box<[Effect]>,
}
impl SerStatus
{
	pub fn into_status(self, type_map: &HashMap<Box<str>, Type>) -> StatusCondition
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
impl Identifiable for SerStatus
{
	fn id(&self) -> Box<str>
	{
		self.id.clone()
	}
}
