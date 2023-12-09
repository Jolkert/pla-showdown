#![allow(dead_code)]

mod data;

use data::{serialization::SerSpecies, Move, Species, Type};
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::data::serialization::{JsonSpecies, SerMove};

type RegMap<T> = HashMap<Box<str>, T>;
lazy_static! {
	static ref TYPE_MAP: RegMap<Type> = register("assets/types/");
	static ref MOVE_MAP: RegMap<Move<'static>> = register::<SerMove>("assets/moves")
		.iter()
		.map(|it| (it.0.to_owned(), it.1.clone().into_move(&TYPE_MAP)))
		.collect();
	static ref SPECIES_MAP: RegMap<Species<'static>> = register::<SerSpecies>("assets/species/")
		.iter()
		.map(|it| (it.0.to_owned(), it.1.clone().into_species(&TYPE_MAP)))
		.collect();
}

fn main()
{
	println!("Pokemon!");
	println!("{:?}", TYPE_MAP["normal"]);
	println!("{:?}", SPECIES_MAP["teddiursa"]);
}

fn json_conversion()
{
	let dir = std::fs::read_dir("assets/json/species/").unwrap();
	for file in dir.map(|it| it.unwrap())
	{
		let filename: String = file
			.file_name()
			.into_string()
			.unwrap()
			.chars()
			.take_while(|it| *it != '.')
			.collect();

		println!("{filename}");
		let json_str = std::fs::read_to_string(file.path()).unwrap();
		let species: JsonSpecies = serde_json::from_str(&json_str).unwrap();
		let species = SerSpecies::from_json_species(species, &filename);
		let toml_str = toml::to_string(&species).unwrap();
		std::fs::write(&(format!("assets/species/{}.toml", filename)), toml_str).unwrap();
	}
}

fn register<T>(dir_path: &str) -> HashMap<Box<str>, T>
where
	T: serde::de::DeserializeOwned + data::Identifiable,
{
	std::fs::read_dir(dir_path)
		.unwrap_or_else(|_| panic!("directory '{}' not found!", dir_path))
		.filter_map(|result| {
			result.ok().and_then(|file| {
				std::fs::read_to_string(file.path())
					.ok()
					.and_then(|data| toml::from_str::<T>(&data).ok().map(|t| (t.id(), t)))
			})
		})
		.collect()
}
