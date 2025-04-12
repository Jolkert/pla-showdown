use std::{
	collections::{HashMap, HashSet},
	path::Path,
};

use pla_showdown::data::{
	IdSet, Identify, Nature,
	serialization::{IntoDeserialized, SerMove, SerSpecies, SerStatus, SerType},
};

fn main()
{
	let types = {
		let deser_types = deserialize_dir::<SerType>("./assets/types").collect::<Vec<_>>();
		let ids = deser_types
			.iter()
			.map(|ty| ty.id.clone())
			.collect::<HashSet<_>>();

		id_set_from(deser_types, &ids)
	};

	let species = id_set_from(deserialize_dir::<SerSpecies>("./assets/species"), &types);
	let moves = id_set_from(deserialize_dir::<SerMove>("./assets/moves"), &types);
	let statuses = id_set_from(deserialize_dir::<SerStatus>("./assets/statuses"), &types);

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

fn id_set_from<'a, I, D, R>(iter: impl IntoIterator<Item = I>, data: &'a R) -> IdSet<D>
where
	I: IntoDeserialized<'a, Deserialized = D, RefData = R>,
	D: Identify,
{
	iter.into_iter()
		.map(|item| item.into_deserialized(data).into())
		.collect()
}
