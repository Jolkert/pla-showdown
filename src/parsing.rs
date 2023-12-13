use crate::data;

use data::{Move, Nature, Pokemon, Species, Stat, StatBlock};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

type RegMap<T> = HashMap<Box<str>, T>;
lazy_static! {
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
	use PokemonParseError as Error;
	let mut lines = data.lines();

	let (species_name, nickname) = find_nickname_and_species(
		lines
			.next()
			.ok_or_else(|| Error(String::from("missing first line!")))?,
	)?;
	let species_name: Box<str> = pokemon_id_from(species_name).into();

	let species = species_map
		.get(&species_name)
		.ok_or_else(|| Error(format!("could not find species '{species_name}'")))?;
	let mut pokemon = Pokemon::new(species).nickname(nickname);

	// TODO: this is fucking awful please do something about this i beg you -morgan 2023-12-11
	for line in lines.map(str::to_lowercase)
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
			pokemon = pokemon.set_shiny(rest == "yes");
		}
		else if let Some(rest) = substring_before_end(&line, " nature")
		{
			pokemon = pokemon.nature(
				*nature_map
					.get(rest)
					.ok_or_else(|| Error(format!("could not find nature '{rest}'")))?,
			);
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
							.replace(' ', "_")
							.into_boxed_str(),
					)
					.ok_or_else(|| Error(format!("could not find move '{rest}'")))?,
			);
		}
	}

	Ok(pokemon)
}

#[derive(Debug)]
pub struct PokemonParseError(String);

fn find_nickname_and_species(string: &str) -> Result<(&str, Option<String>), PokemonParseError>
{
	let last_open = find_last('(', string);
	let last_close = find_last(')', string);

	if last_open > last_close
	{
		Err(PokemonParseError(format!(
			"could not parse nickname and species from {string}"
		)))
	}
	else if let (Some(open), Some(close)) = (last_open, last_close)
	{
		let species = string[(open + 1)..close].trim();
		let name = string[..open].trim();

		Ok((species, Some(String::from(name))))
	}
	else
	{
		Ok((string.trim(), None))
	}
}

fn pokemon_id_from(string: &str) -> String
{
	string
		.trim()
		.to_lowercase()
		.chars()
		.map(|it| {
			if it.is_ascii() && (it.is_alphanumeric() || it == '-')
			{
				it
			}
			else
			{
				'_'
			}
		})
		.collect::<String>()
		.trim_matches('_')
		.replace("__", "_")
}

fn find_last(ch: char, string: &str) -> Option<usize>
{
	string
		.char_indices()
		.filter(|it| it.1 == ch)
		.last()
		.map(|it| it.0)
}

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
			PokemonParseError(format!("could not interpret effor level from '{block}'"))
		})?;
		let value: i32 = captures["val"].parse().unwrap();
		let stat: Stat = captures["stat"].parse().unwrap();
		stat_map.insert(stat, value);
	}

	Ok(StatBlock::for_each_stat(|stat| {
		stat_map.get(&stat).map_or(10, i32::clone)
	}))
}
