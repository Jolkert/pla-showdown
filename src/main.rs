#![allow(dead_code)]

mod data;
mod parsing;

use data::{
	serialization::{SerMove, SerSpecies, SerStatus},
	Move, Nature, Species, StatusCondition, Type,
};
use lazy_static::lazy_static;
use std::collections::HashMap;

type RegMap<T> = HashMap<Box<str>, T>;
lazy_static! {
	static ref TYPE_MAP: RegMap<Type> = register("assets/types/");
	static ref MOVE_MAP: RegMap<Move<'static>> = register::<SerMove>("assets/moves/")
		.iter()
		.map(|it| (it.0.to_owned(), it.1.clone().into_move(&TYPE_MAP)))
		.collect();
	static ref SPECIES_MAP: RegMap<Species<'static>> = register::<SerSpecies>("assets/species/")
		.iter()
		.map(|it| (it.0.to_owned(), it.1.clone().into_species(&TYPE_MAP)))
		.collect();
	static ref STATUS_MAP: RegMap<StatusCondition<'static>> =
		register::<SerStatus>("assets/statuses/")
			.iter()
			.map(|it| (it.0.to_owned(), it.1.clone().into_status(&TYPE_MAP)))
			.collect();
	static ref NATURE_MAP: RegMap<Nature> =
		toml::from_str(&std::fs::read_to_string("assets/natures.toml").unwrap()).unwrap();
}

fn main()
{
	println!("pokemon!");
}

fn register<T>(dir_path: &str) -> RegMap<T>
where
	T: serde::de::DeserializeOwned + data::Identifiable,
{
	std::fs::read_dir(dir_path)
		.unwrap_or_else(|_| panic!("directory '{dir_path}' not found!"))
		.filter_map(|result| {
			result.ok().and_then(|file| {
				std::fs::read_to_string(file.path())
					.ok()
					.and_then(|data| toml::from_str::<T>(&data).ok().map(|t| (t.id(), t)))
			})
		})
		.collect()
}
