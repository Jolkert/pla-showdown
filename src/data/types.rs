use crate::data;

use data::Identifiable;

#[derive(Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Type
{
	pub id: Box<str>,
	#[serde(rename = "weaknesses")]
	pub weakness_ids: Box<[Box<str>]>,
	#[serde(rename = "resistances")]
	pub resistance_ids: Box<[Box<str>]>,
	#[serde(rename = "immunities")]
	pub immunity_ids: Box<[Box<str>]>,
}

impl Identifiable for Type
{
	fn id(&self) -> Box<str>
	{
		self.id.clone()
	}
}

#[derive(Debug)]
pub struct TypePair<'a>(pub &'a Type, pub Option<&'a Type>);
