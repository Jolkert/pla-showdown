use std::{collections::HashSet, path::Path};

use color_eyre::eyre;
use pla_showdown::data::{
	Data, IdSet, Identify,
	serialization::{IntoDeserialized, SerMove, SerSpecies, SerStatus, SerType},
};

pub fn initialize_data() -> eyre::Result<Data>
{
	log::info!("Loading types...");
	let types = {
		let deser_types = deserialize_dir::<SerType>("./assets/types")?;
		let ids = deser_types
			.iter()
			.map(|ty| ty.id.clone())
			.collect::<HashSet<_>>();

		id_set_from(deser_types, &ids)
	};

	log::info!("Loading species...");
	let species = id_set_from(deserialize_dir::<SerSpecies>("./assets/species")?, &types);

	log::info!("Loading moves...");
	let moves = id_set_from(deserialize_dir::<SerMove>("./assets/moves")?, &types);

	log::info!("Loading statuses...");
	let statuses = id_set_from(deserialize_dir::<SerStatus>("./assets/statuses")?, &types);

	Ok(Data {
		types,
		species,
		moves,
		statuses,
		natures: toml::from_str(&std::fs::read_to_string("./assets/natures.toml")?)?,
	})
}

fn deserialize_dir<T: serde::de::DeserializeOwned>(
	path: impl AsRef<Path>,
) -> std::io::Result<Vec<T>>
{
	std::fs::read_dir(&path)?
		.filter_map(|dir_entry| {
			dir_entry.map_or_else(
				|err| Some(Err(err)),
				|file| {
					Ok(std::fs::read_to_string(file.path())
						.ok()
						.and_then(|toml_str| {
							toml::from_str::<T>(&toml_str)
								.inspect_err(|err| {
									log::warn!(
										"Failed to deserialize file {}\n{err}",
										file.file_name().display()
									);
								})
								.ok()
						}))
					.transpose()
				},
			)
		})
		.collect()

	// std::fs::read_dir(&path)?.filter_map(|result| {
	// 	result.ok().and_then(|file| {
	// 		std::fs::read_to_string(file.path())
	// 			.ok()
	// 			.and_then(|toml_str| {
	// 				toml::from_str::<T>(&toml_str)
	// 					.inspect_err(|err| {
	// 						log::warn!(
	// 							"Failed to deserialize file {}\n{err}",
	// 							file.file_name().display()
	// 						);
	// 					})
	// 					.ok()
	// 			})
	// 	})
	// })
}

fn id_set_from<'a, S, D, R>(iter: impl IntoIterator<Item = S>, data: &'a R) -> IdSet<D>
where
	S: IntoDeserialized<'a, Deserialized = D, RefData = R>,
	D: Identify,
{
	iter.into_iter()
		.map(|item| item.into_deserialized(data))
		.collect()
}
