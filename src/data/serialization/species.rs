use crate::data::{Identifiable, Species, StatBlock, Type, TypePair};
use std::collections::HashMap;

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
}
impl Identifiable for SerSpecies
{
	fn id(&self) -> Box<str>
	{
		self.id.clone()
	}
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
