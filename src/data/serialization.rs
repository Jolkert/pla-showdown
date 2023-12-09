pub use moves::*;
pub use species::*;

mod species
{
	use std::collections::HashMap;

	use crate::data::{Identifiable, Species, StatBlock, Type, TypePair};

	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
	pub struct SerSpecies
	{
		pub id: Box<str>,
		pub base_stats: StatBlock,
		#[serde(with = "deserialize_species_types")]
		pub types: (Box<str>, Option<Box<str>>),
	}
	impl SerSpecies
	{
		pub fn into_species(self, type_map: &HashMap<Box<str>, Type>) -> Species
		{
			Species {
				id: self.id,
				base_stats: self.base_stats,
				types: TypePair(
					type_map.get(&self.types.0).unwrap(),
					self.types.1.and_then(|it| type_map.get(&it)),
				),
			}
		}

		pub fn from_json_species(json_species: JsonSpecies, name: &str) -> Self
		{
			let mut json_types = json_species.types.iter();
			let type1 = json_types.next().unwrap().clone();
			let type2 = json_types.next().cloned();

			Self {
				id: name.to_owned().into(),
				base_stats: json_species.base_stats,
				types: (type1.into(), type2.map(|it| it.into())),
			}
		}
	}
	impl Identifiable for SerSpecies
	{
		fn id(&self) -> Box<str>
		{
			self.id.clone()
		}
	}

	#[derive(Debug, serde::Serialize, serde::Deserialize)]
	pub struct JsonSpecies
	{
		#[serde(alias = "baseStats")]
		pub base_stats: StatBlock,
		pub types: Vec<String>,
	}

	mod deserialize_species_types
	{
		use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serializer};

		type SerdeType = Box<[Box<str>]>;
		type RustType = (Box<str>, Option<Box<str>>);

		pub fn serialize<S>(value: &RustType, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = if value.1.is_some() { 2 } else { 1 };
			let mut seq = serializer.serialize_seq(Some(len))?;
			seq.serialize_element(&value.0)?;
			if let Some(str) = &value.1
			{
				seq.serialize_element(str)?;
			}
			seq.end()
		}

		pub fn deserialize<'de, D>(deserializer: D) -> Result<RustType, D::Error>
		where
			D: Deserializer<'de>,
		{
			let vec = SerdeType::deserialize(deserializer)?;
			// TODO: if the first type is missing, this function will panic instead of returning Err(D::Error)
			// this is fine for now, but should probably be fixed -morgan 2023-12-08
			Ok((vec[0].clone(), vec.get(1).cloned()))
		}
	}
}

mod moves
{
	use std::collections::HashMap;

	use crate::data::{Category, Identifiable, Move, StyleTriad, Type};

	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
}
