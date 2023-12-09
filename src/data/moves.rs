use crate::data::Type;

pub use style::*;

#[derive(Debug)]
pub struct Move<'a>
{
	pub id: Box<str>,
	pub move_type: &'a Type,
	pub category: MoveCategory,
	pub pp: u32,
	pub power: StyleTriad<i32>,
	pub accuracy: StyleTriad<i32>,
	pub user_action_time: StyleTriad<i32>,
	pub target_action_time: StyleTriad<i32>,
	pub crit_stage: StyleTriad<i32>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MoveCategory
{
	Physical,
	Special,
	Status,
}
impl MoveCategory
{
	pub fn is_damaging(&self) -> bool
	{
		!matches!(self, Self::Status)
	}
}

mod style
{
	#[derive(Debug, Clone, Copy)]
	pub enum MoveStyle
	{
		Normal,
		Agile,
		Strong,
	}

	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
	pub struct StyleTriad<T>
	where
		T: Copy,
	{
		pub normal: T,
		pub agile: T,
		pub strong: T,
	}
	impl<T> std::ops::Index<MoveStyle> for StyleTriad<T>
	where
		T: Copy,
	{
		type Output = T;
		fn index(&self, index: MoveStyle) -> &Self::Output
		{
			match index
			{
				MoveStyle::Normal => &self.normal,
				MoveStyle::Agile => &self.agile,
				MoveStyle::Strong => &self.strong,
			}
		}
	}
	impl<T> StyleTriad<T>
	where
		T: Copy,
	{
		fn new(normal: T, agile: T, strong: T) -> Self
		{
			Self {
				normal,
				agile,
				strong,
			}
		}

		fn all(val: T) -> Self
		{
			StyleTriad {
				normal: val,
				agile: val,
				strong: val,
			}
		}
	}
}
