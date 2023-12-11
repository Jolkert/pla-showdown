use crate::data::{Move, Nature, Pokemon, Species, Stat, StatBlock};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

type RegMap<T> = HashMap<Box<str>, T>;
lazy_static! {
	static ref NICKNAME_SPECIES_REGEX: Regex =
		Regex::new(r"(?<name_or_species>\S+)(\s*\((?<species>\S+)\))?").unwrap();
	static ref EFFORT_REGEX: Regex =
		Regex::new(r"(?<val>\d+)\s*(?<stat>(hp|atk|def|spa|spd|spe))").unwrap();
}

pub fn deserialize_pokemon<'a>(
	data: &str,
	species_map: &'a RegMap<Species<'a>>,
	move_map: &'a RegMap<Move<'a>>,
	nature_map: &RegMap<Nature>,
) -> Result<Pokemon<'a>, PokemonParseError>
{
	// TODO: this is fucking awful please do something about this i beg you -morgan 2023-12-11
	use PokemonParseError as Error;

	let mut lines = data.lines();
	// species & nickname
	let captures = NICKNAME_SPECIES_REGEX
		.captures(
			lines
				.next()
				.ok_or_else(|| Error(String::from("missing first line!")))?,
		)
		.ok_or_else(|| Error(String::from("could not find species in first line!")))?;

	let name_or_species: Box<str> = captures["name_or_species"].into();
	let (species_name, nickname) = if let Some(species) = captures.name("species")
	{
		(
			pokemon_id_from(&species.as_str()).into(),
			Some(String::from(name_or_species)),
		)
	}
	else
	{
		(name_or_species, None)
	};

	let species = species_map
		.get(&species_name)
		.ok_or_else(|| Error(format!("could not find species \'{species_name}\'")))?;
	let mut pokemon = Pokemon::new(species).nickname(nickname);

	// the rest of the pokemon
	for line in lines.map(|it| it.to_lowercase())
	{
		if let Some(rest) = substring_after_start(&line, "level: ")
		{
			pokemon = pokemon.level(
				rest.parse::<u8>()
					.map_err(|_| Error(String::from("could not parse level!")))?,
			);
		}
		else if let Some(rest) = substring_after_start(&line, "shiny: ")
		{
			pokemon = pokemon.set_shiny(rest == "yes")
		}
		else if let Some(rest) = substring_before_end(&line, " nature")
		{
			pokemon = pokemon.nature(
				nature_map
					.get(rest)
					.ok_or_else(|| Error(format!("could not find nature \'{rest}\'")))?
					.clone(),
			)
		}
		else if let Some(rest) = substring_after_start(&line, "els: ")
		{
			pokemon = pokemon.effort_levels(parse_effort_levels(rest)?);
		}
		else if let Some(rest) = substring_after_start(&line, "- ")
		{
			pokemon = pokemon.add_move(
				move_map
					.get(
						&rest
							.to_lowercase()
							.trim()
							.replace(" ", "_")
							.into_boxed_str(),
					)
					.ok_or_else(|| Error(format!("could not find move \'{rest}\'")))?,
			);
		}
	}

	Ok(pokemon)
}

#[derive(Debug)]
pub struct PokemonParseError(String);

fn substring_after_start<'a>(string: &'a str, pattern: &str) -> Option<&'a str>
{
	let index = string.find(pattern)?;
	if index == 0
	{
		Some(&string[(index + pattern.len())..])
	}
	else
	{
		None
	}
}

fn substring_before_end<'a>(string: &'a str, pattern: &str) -> Option<&'a str>
{
	let index = string.find(pattern)?;
	if index + pattern.len() == string.len()
	{
		Some(&string[..index])
	}
	else
	{
		None
	}
}

fn parse_effort_levels(string: &str) -> Result<StatBlock, PokemonParseError>
{
	let mut stat_map = HashMap::new();
	let blocks = string.split('/').map(str::trim);
	for block in blocks
	{
		let captures = EFFORT_REGEX.captures(block).ok_or_else(|| {
			PokemonParseError(format!("could not interpret effor level from \'{block}\'"))
		})?;
		let value: i32 = captures["val"].parse().unwrap();
		let stat: Stat = captures["stat"].parse().unwrap();
		stat_map.insert(stat, value);
	}

	Ok(StatBlock {
		hp: stat_map.get(&Stat::Hp).map(i32::clone).unwrap_or(10),
		atk: stat_map.get(&Stat::Atk).map(i32::clone).unwrap_or(10),
		def: stat_map.get(&Stat::Def).map(i32::clone).unwrap_or(10),
		spatk: stat_map.get(&Stat::SpAtk).map(i32::clone).unwrap_or(10),
		spdef: stat_map.get(&Stat::SpDef).map(i32::clone).unwrap_or(10),
		spe: stat_map.get(&Stat::Spe).map(i32::clone).unwrap_or(10),
	})
}

fn pokemon_id_from(string: &str) -> String
{
	string
		.to_lowercase()
		.chars()
		.map(|it| {
			if it.is_ascii() && it.is_alphanumeric()
			{
				it
			}
			else
			{
				'_'
			}
		})
		.collect::<String>()
		.replace("__", "_")
}
