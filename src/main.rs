use std::{
	collections::{HashMap, HashSet},
	path::Path,
};

use pla_showdown::data::{
	IdSet, Nature,
	serialization::{SerMove, SerSpecies, SerStatus, SerType},
};

fn main()
{
	// TODO: make the `into_*` conversion mapping generic
	// - morgan 2025-04-12

	let types = {
		let deser_types = deserialize_dir::<SerType>("./assets/types").collect::<Vec<_>>();

		let ids = deser_types
			.iter()
			.map(|ty| ty.id.clone())
			.collect::<HashSet<_>>();

		deser_types
			.into_iter()
			.map(|ser| ser.into_type(&ids).into())
			.collect::<IdSet<_>>()
	};

	let species = deserialize_dir::<SerSpecies>("./assets/species")
		.map(|ser| ser.into_species(&types).into())
		.collect::<IdSet<_>>();

	let moves = deserialize_dir::<SerMove>("./assets/moves")
		.map(|ser| ser.into_move(&types).into())
		.collect::<IdSet<_>>();

	let statuses = deserialize_dir::<SerStatus>("./assets/statuses")
		.map(|ser| ser.into_status(&types).into())
		.collect::<IdSet<_>>();

	let natures = toml::from_str::<HashMap<Box<str>, Nature>>(
		&std::fs::read_to_string("./assets/natures.toml").unwrap(),
	)
	.unwrap();

	println!("Types: {}", types.len());
	println!("Species: {}", species.len());
	println!("Moves: {}", moves.len());
	println!("Statuses: {}", statuses.len());
	println!("Natures: {}", natures.len());
}

fn deserialize_dir<T: serde::de::DeserializeOwned>(
	path: impl AsRef<Path>,
) -> impl Iterator<Item = T>
{
	std::fs::read_dir(&path)
		.unwrap_or_else(|err| {
			panic!(
				"Could find directory `{}`!\n{err}",
				path.as_ref().to_string_lossy()
			)
		})
		.filter_map(|result| {
			result.ok().and_then(|file| {
				std::fs::read_to_string(file.path())
					.ok()
					.and_then(|toml_str| {
						toml::from_str::<T>(&toml_str)
							.inspect_err(|err| {
								eprintln!(
									"Failed to deserialize file {:#?}\n{err}",
									file.file_name()
								)
							})
							.ok()
					})
			})
		})
}
