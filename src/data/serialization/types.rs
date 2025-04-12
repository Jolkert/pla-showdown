use std::{collections::HashSet, rc::Rc};

use crate::{
	BoxSlice, BoxStr,
	data::{Identifiable, Type},
};

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

impl SerType
{
	pub fn into_type(self, type_ids: &HashSet<Rc<str>>) -> Identifiable<Type>
	{
		Type {
			id: self.id,
			weakness_ids: self
				.weakness_ids
				.into_iter()
				.filter_map(|weakness| type_ids.get(&*weakness).map(Rc::downgrade))
				.collect(),
			resistance_ids: self
				.resistance_ids
				.into_iter()
				.filter_map(|resistance| type_ids.get(&*resistance).map(Rc::downgrade))
				.collect(),
			immunity_ids: self
				.immunity_ids
				.into_iter()
				.filter_map(|immunity| type_ids.get(&*immunity).map(Rc::downgrade))
				.collect(),
		}
		.into()
	}
}
