use super::IntoDeserialized;
use crate::{
	BoxSlice, BoxStr,
	data::{Effect, IdSet, StatusCondition, Type, Volatility},
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
impl<'a> IntoDeserialized<'a> for SerStatus
{
	type Deserialized = StatusCondition<'a>;
	type RefData = IdSet<Type>;

	fn into_deserialized(self, data: &'a Self::RefData) -> Self::Deserialized
	{
		StatusCondition {
			id: self.id,
			volatility: self.volatility,
			effects: self.effects,
			immune_types: self
				.immune_type_ids
				.into_iter()
				.map(|id| data.get(&*id).unwrap().ref_inner())
				.collect(),
		}
	}
}
