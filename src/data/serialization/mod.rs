mod moves;
mod species;
mod status;

pub use moves::*;
pub use species::*;
pub use status::*;

fn empty_slice<T>() -> Box<[T]>
{
	Box::new([])
}
