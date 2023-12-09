use crate::data::{Category, Identifiable, Move, MoveEffect, StyleTriad, Type};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SerMove
{
	pub id: Box<str>,
	#[serde(rename = "type")]
	pub move_type: Box<str>,
	pub category: Category,
	pub pp: u32,
	pub power: StyleTriad<i32>,
	pub accuracy: StyleTriad<i32>,
	pub user_action_time: StyleTriad<i32>,
	pub target_action_time: StyleTriad<i32>,
	pub crit_stage: StyleTriad<i32>,
	pub effects: Box<[MoveEffect]>,
}
impl SerMove
{
	pub fn into_move(self, type_map: &HashMap<Box<str>, Type>) -> Move
	{
		Move {
			id: self.id,
			move_type: type_map.get(&self.move_type).unwrap(),
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
impl Identifiable for SerMove
{
	fn id(&self) -> Box<str>
	{
		self.id.clone()
	}
}
