use std::{
	collections::{HashMap, HashSet},
	path::Path,
	rc::Rc,
};

use eframe::egui;
use pla_showdown::data::{
	Data, IdSet, Identify, Move, Nature, Species, Type,
	serialization::{IntoDeserialized, SerMove, SerSpecies, SerStatus, SerType},
};

fn main()
{
	let _ = dotenv::dotenv();
	env_logger::init();

	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
		..Default::default()
	};

	let _ = eframe::run_native(
		"pla-showdown",
		options,
		Box::new(|_| {
			Ok(Box::from(Showdown {
				data: Rc::from(initialize_data()),
			}))
		}),
	);
}

struct Showdown
{
	data: Rc<Data>,
}
impl eframe::App for Showdown
{
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
	{
		egui::CentralPanel::default().show(ctx, |ui| {
			self.debug_panel(ui);
		});
	}
}

impl Showdown
{
	fn data(&self) -> Rc<Data>
	{
		self.data.clone()
	}

	fn debug_panel(&mut self, ui: &mut egui::Ui)
	{
		egui::ScrollArea::vertical()
			.auto_shrink([false, false])
			.show(ui, |ui| {
				ui.collapsing("Types", |ui| {
					for typ in itertools::sorted(self.data().types.iter())
					{
						self.show_type(ui, typ);
					}
				});
				ui.collapsing("Pokemon", |ui| {
					for mon in itertools::sorted(self.data().species.iter())
					{
						self.show_species(ui, mon)
					}
				});
				ui.collapsing("Moves", |ui| {
					for mov in itertools::sorted(self.data().moves.iter())
					{
						self.show_move(ui, mov)
					}
				});
			});
	}

	fn show_type(&mut self, ui: &mut egui::Ui, typ: &Type)
	{
		ui.vertical(|ui| {
			ui.collapsing(typ.id(), |ui| {
				ui.label(format!(
					"weak: [{}]",
					typ.weakness_ids
						.iter()
						.filter_map(|id| { id.upgrade().map(|rc| String::from(rc.as_ref())) })
						.collect::<Vec<_>>()
						.join(", ")
				));
				ui.label(format!(
					"resist: [{}]",
					typ.resistance_ids
						.iter()
						.filter_map(|id| { id.upgrade().map(|rc| String::from(rc.as_ref())) })
						.collect::<Vec<_>>()
						.join(", ")
				));
				ui.label(format!(
					"immune: [{}]",
					typ.immunity_ids
						.iter()
						.filter_map(|id| { id.upgrade().map(|rc| String::from(rc.as_ref())) })
						.collect::<Vec<_>>()
						.join(", ")
				));
			})
		});
	}

	fn show_species(&mut self, ui: &mut egui::Ui, species: &Species)
	{
		ui.vertical(|ui| {
			ui.collapsing(species.id(), |ui| {
				ui.label(species.types.to_string());
				ui.label(species.base_stats.to_string());
			})
		});
	}

	fn show_move(&mut self, ui: &mut egui::Ui, mov: &Move)
	{
		ui.vertical(|ui| {
			ui.collapsing(mov.id(), |ui| {
				ui.label(mov.move_type().id());
				ui.label(mov.category.to_string());
				ui.label(format!("{} pp", mov.pp));
				ui.horizontal(|ui| {
					ui.label("Power:");
					if mov.power.are_all_equal() && mov.power.regular == 0
					{
						ui.label("—");
					}
					else
					{
						ui.label(mov.power.to_string());
					}
				});
				ui.horizontal(|ui| {
					ui.label("Acc:");
					if mov.accuracy.are_all_equal() && mov.accuracy.regular == 101
					{
						ui.label("—");
					}
					else
					{
						ui.label(mov.accuracy.to_string());
					}
				});
				if !(mov.user_action_time.are_all_equal() && mov.user_action_time.regular == 0)
				{
					ui.horizontal(|ui| {
						ui.label("AT mod (user):");
						ui.label(mov.user_action_time.to_string());
					});
				}

				if !(mov.target_action_time.are_all_equal() && mov.target_action_time.regular == 0)
				{
					ui.horizontal(|ui| {
						ui.label("AT mod (target):");
						ui.label(mov.target_action_time.to_string());
					});
				}
				if !(mov.crit_stage.are_all_equal() && mov.crit_stage.regular == 0)
				{
					ui.horizontal(|ui| {
						ui.label("Crit:");
						ui.label(mov.crit_stage.to_string());
					});
				}

				if !mov.effects.is_empty()
				{
					ui.label("Secondary Effects:");
					for effect in &mov.effects
					{
						ui.label(effect.to_string());
					}
				}
			})
		});
	}
}

fn initialize_data() -> Data
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

	Data {
		types,
		species,
		moves,
		statuses,
		natures: toml::from_str::<HashMap<Box<str>, Nature>>(
			&std::fs::read_to_string("./assets/natures.toml").unwrap(),
		)
		.unwrap(),
	}
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
								log::error!(
									"Failed to deserialize file {:#?}\n{err}",
									file.file_name()
								)
							})
							.ok()
					})
			})
		})
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
