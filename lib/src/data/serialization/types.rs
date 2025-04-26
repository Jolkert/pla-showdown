use std::{collections::HashSet, rc::Rc};

use tap::TapOptional;

use super::IntoDeserialized;
use crate::{BoxSlice, BoxStr, data::Type};

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

impl IntoDeserialized<'_> for SerType
{
	type Deserialized = Rc<Type>;
	type RefData = HashSet<Rc<str>>;

	fn into_deserialized(self, data: &Self::RefData) -> Self::Deserialized
	{
		// `id` has to be at the bottom because the order actually matters here
		// the tap branch where we log a warning wont compile with `id` at the top because
		// `self.id` wouldve already been moved out of, meaning we couldnt use it in the warning
		// message. its very silly that the order matters but here we are
		// -morgan 2025-04-25
		Rc::from(Type {
			weakness_ids: self
				.weakness_ids
				.into_iter()
				.filter_map(|weakness| {
					data.get(weakness.as_ref())
						.map(Rc::downgrade)
						.tap_none(|| log::warn!("{} could not find weakness {}", self.id, weakness))
				})
				.collect(),

			resistance_ids: self
				.resistance_ids
				.into_iter()
				.filter_map(|resistance| {
					data.get(resistance.as_ref())
						.map(Rc::downgrade)
						.tap_none(|| {
							log::warn!("{} could not find resistance {}", self.id, resistance);
						})
				})
				.collect(),

			immunity_ids: self
				.immunity_ids
				.into_iter()
				.filter_map(|immunity| {
					data.get(immunity.as_ref())
						.map(Rc::downgrade)
						.tap_none(|| log::warn!("{} could not find immunity {}", self.id, immunity))
				})
				.collect(),
			id: self.id,
		})
	}
}
