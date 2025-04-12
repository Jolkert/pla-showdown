use crate::{BoxSlice, BoxStr, data};

use data::{Category, Move, MoveEffect, StyleTriad, Type};
use std::collections::HashMap;

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
impl SerMove
{
	pub fn into_move(self, type_map: &HashMap<BoxStr, Type>) -> Move
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
