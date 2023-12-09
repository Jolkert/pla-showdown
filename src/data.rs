mod moves;
mod pokemon;
pub mod serialization;
mod stats;
mod status;
mod types;

pub use moves::*;
pub use pokemon::*;
pub use stats::*;
pub use status::*;
pub use types::*;

pub trait Identifiable
{
	fn id(&self) -> Box<str>;
}
