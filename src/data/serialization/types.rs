use std::{collections::HashSet, rc::Rc};

use crate::{
	BoxSlice, BoxStr,
	data::{Identifiable, Type},
};

use super::IntoDeserialized;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerType
{
	pub id: Rc<str>,

	#[serde(rename = "weaknesses")]
	pub weakness_ids: BoxSlice<BoxStr>,
	#[serde(rename = "resistances")]
	pub resistance_ids: BoxSlice<BoxStr>,
	#[serde(rename = "immunities")]
	pub immunity_ids: BoxSlice<BoxStr>,
}

impl<'a> IntoDeserialized<'a> for SerType
{
	type Deserialized = Type;
	type RefData = HashSet<Rc<str>>;

	fn into_deserialized(self, data: &Self::RefData) -> Self::Deserialized
	{
		Type {
			id: self.id,
			weakness_ids: self
				.weakness_ids
				.into_iter()
				.filter_map(|weakness| data.get(&*weakness).map(Rc::downgrade))
				.collect(),
			resistance_ids: self
				.resistance_ids
				.into_iter()
				.filter_map(|resistance| data.get(&*resistance).map(Rc::downgrade))
				.collect(),
			immunity_ids: self
				.immunity_ids
				.into_iter()
				.filter_map(|immunity| data.get(&*immunity).map(Rc::downgrade))
				.collect(),
		}
	}
}
