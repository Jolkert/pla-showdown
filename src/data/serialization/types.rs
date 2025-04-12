use std::rc::Rc;

use crate::{BoxSlice, BoxStr};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerType
{
	id: Rc<str>,

	#[serde(rename = "weaknesses")]
	weakness_ids: BoxSlice<BoxStr>,
	#[serde(rename = "resistances")]
	resistance_ids: BoxSlice<BoxStr>,
	#[serde(rename = "immunities")]
	immunity_ids: BoxSlice<BoxStr>,
}
