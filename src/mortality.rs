use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use humantime::format_duration;
use std::{f32::consts::E, time::Instant};
use rnglib::{RNG, Language};
use rand::Rng;
use crate::player::*;

use crate::constants::GAMETIME_SCALE;

pub struct MortalityPlugin;

impl Plugin for MortalityPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<BirthEvent>()
        .add_event::<DeathEvent>()
        .add_systems(Startup, adam_born)
        .add_systems(Update, show_age)
        .add_systems(Update, (death_harvest, show_cementry, death_event_handler, birth_event_handler))
        .add_systems(Update, respawn_button);
    }
}

#[derive(Debug, Component)]
pub struct DateOfBirth(Instant);

#[derive(Debug, Component)]
pub struct Mortality;

#[derive(Debug, Component)]
pub struct DateOfDeath(Instant);

// Events

#[derive(Debug, Event)]
pub struct DeathEvent {
    entity: String, // TODO: How to pass here any Mortality object?
}

#[derive(Debug, Event)]
pub struct BirthEvent;

// Systems

// Born of the first player in the world
pub fn adam_born(
    mut birth_event: EventWriter<BirthEvent>,
) {
    birth_event.send(BirthEvent);
}

pub fn show_age(mut contexts: EguiContexts, mut query: Query<(&Player, &DateOfBirth), Without<DateOfDeath>>) {
    for (player, dob) in &mut query {
        egui::Window::new(&player.name).show(contexts.ctx_mut(), |ui| {
            let age: String = "Возраст: ".to_owned()
                + &format_duration(dob.0.elapsed() * GAMETIME_SCALE as u32).to_string();
            // TODO: Refactor it.
            if let Some((first, _)) = age.split_once("months") {
                ui.label(first.to_owned() + "months");
            } else if let Some((first, _)) = age.split_once("month") {
                ui.label(first.to_owned() + "month");
            } else if let Some((first, _)) = age.split_once("year") {
                ui.label(first.to_owned() + "year");
            } else {
                ui.label(age);
            }
        });
    }
}
pub fn death_harvest(
    mut query: Query<(&Player, &DateOfBirth, &Mortality), Without<DateOfDeath>>,
    mut death_event: EventWriter<DeathEvent>,
) {
    for (player, dob, _) in &mut query {
        let age: f32 = Instant::now().duration_since(dob.0).as_secs() as f32; // From 0 to 100
        //let coefficient: f64 = 3.720076e-37;
        let coefficient: f64 = 3.720076e-2; // For debug: average 5 years before death. TODO: delete it.
        let mut death_chance: f64 = coefficient * E.powf(age) as f64;
        if death_chance >= 100. {
            death_chance = 100.
        }

        // Check whether the death occured based on its probabiliy (>90 years 100% chance to die)
        let mut r = rand::thread_rng();
        if r.gen_range(0.0..100.0) <= death_chance {
            debug!("death_harvest");
            death_event.send(DeathEvent {
                entity: player.name.to_string(),
            });
        }
    }
}

pub fn show_cementry(
    mut contexts: EguiContexts,
    mut query: Query<(&Player, &DateOfBirth, &DateOfDeath), With<DateOfDeath>>,
) {
    for (player, dob, dod) in &mut query {
        egui::Window::new(&player.name).show(contexts.ctx_mut(), |ui| {
            let age: String = "Умер в возрасте: ".to_owned()
                + &format_duration(dod.0.duration_since(dob.0) * GAMETIME_SCALE as u32).to_string();
            // TODO: Refactor it.
            if let Some((first, _)) = age.split_once("months") {
                ui.label(first.to_owned() + "months");
            } else if let Some((first, _)) = age.split_once("month") {
                ui.label(first.to_owned() + "month");
            } else if let Some((first, _)) = age.split_once("year") {
                ui.label(first.to_owned() + "year");
            } else {
                ui.label(age);
            }
        });
    }
}

pub fn death_event_handler(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    query: Query<(Entity, &Player), (With<Player>, With<DateOfBirth>, Without<DateOfDeath>)>,
) {
    for death_event in death_events.read() {
        debug!("death_animation");
        for (entity_id, player) in query.iter() {
            dbg!(entity_id);
            //info!(player_id);
            if player.name == death_event.entity {
                commands.entity(entity_id)
                    .insert(DateOfDeath(Instant::now()));
            }
        }
    }
}

pub fn birth_event_handler(
    mut commands: Commands,
    mut birth_events: EventReader<BirthEvent>,
) {
    for _ in birth_events.read() {
        let first = RNG::try_from(&Language::Elven).unwrap();
        let last = RNG::try_from(&Language::Elven).unwrap();

        commands.spawn((
            Player{
                name: first.generate_name() + " " + &last.generate_name(),
            },
            DateOfBirth(Instant::now()),
            Mortality,
        ));
    }
}

pub fn respawn_button(
    mut contexts: EguiContexts,
    mut birth_event: EventWriter<BirthEvent>,
) {
    let mut new_player = false;
    let ctx = contexts.ctx_mut();
    egui::SidePanel::left("side_panel")
        .show(ctx, |ui| {
            ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.horizontal(|ui| {
                new_player = ui.button("New player").clicked();
            });
        });
    if new_player {
        birth_event.send(BirthEvent);
    }
}