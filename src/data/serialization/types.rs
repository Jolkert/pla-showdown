use std::rc::Rc;

use crate::{BoxSlice, BoxStr};

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
