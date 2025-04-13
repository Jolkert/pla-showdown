use super::IntoDeserialized;
use crate::{
	BoxSlice, BoxStr,
	data::{Category, IdSet, Move, MoveEffect, StyleTriad, Type},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SerMove
{
	pub id: BoxStr,
	#[serde(rename = "type")]
	pub move_type: BoxStr,
	pub category: Category,
	pub pp: u32,
	pub power: StyleTriad,
	pub accuracy: StyleTriad,
	pub user_action_time: StyleTriad,
	pub target_action_time: StyleTriad,
	pub crit_stage: StyleTriad,
	pub effects: BoxSlice<MoveEffect>,
}
impl<'a> IntoDeserialized<'a> for SerMove
{
	type Deserialized = Move<'a>;
	type RefData = IdSet<Type>;

	fn into_deserialized(self, data: &'a Self::RefData) -> Self::Deserialized
	{
		Move {
			id: self.id,
			move_type: data.get(&*self.move_type).unwrap(),
			category: self.category,
			pp: self.pp,
			power: self.power,
			accuracy: self.accuracy,
			user_action_time: self.user_action_time,
			target_action_time: self.target_action_time,
			crit_stage: self.crit_stage,
			effects: self.effects,
		}
	}
}
