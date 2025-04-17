use super::IntoDeserialized;
use crate::{
	BoxStr,
	data::{IdSet, Species, StatBlock, Type, TypePair},
};
use std::rc::Rc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerSpecies
{
	pub id: BoxStr,
	pub base_stats: StatBlock,
	#[serde(with = "deserialize_species_types")]
	pub types: (BoxStr, Option<BoxStr>),
}
impl<'a> IntoDeserialized<'a> for SerSpecies
{
	type Deserialized = Species;
	type RefData = IdSet<Rc<Type>>;

	fn into_deserialized(self, data: &'a Self::RefData) -> Self::Deserialized
	{
		Species {
			id: self.id,
			base_stats: self.base_stats,
			types: TypePair::from((
				(**data.get(&*self.types.0).unwrap()).clone(),
				self.types
					.1
					.and_then(|id| data.get(&*id))
					.map(|typ| (**typ).clone()),
			)),
		}
	}
}

mod deserialize_species_types
{
	use serde::{Deserialize, Deserializer, Serializer, ser::SerializeSeq};

	use crate::{BoxSlice, BoxStr};

	type IntermediateType = BoxSlice<BoxStr>;
	type TargetType = (BoxStr, Option<BoxStr>);

	pub fn serialize<S>(value: &TargetType, serializer: S) -> Result<S::Ok, S::Error>
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

	pub fn deserialize<'de, D>(deserializer: D) -> Result<TargetType, D::Error>
	where
		D: Deserializer<'de>,
	{
		let vec = IntermediateType::deserialize(deserializer)?;
		// TODO: if the first type is missing, this function will panic instead of returning Err(D::Error)
		// this is fine for now, but should probably be fixed -morgan 2023-12-08
		Ok((vec[0].clone(), vec.get(1).cloned()))
	}
}
