mod constants;
mod ui;

use std::collections::HashMap;
use std::time::Duration;
use constants::*;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use battle_plugin::*;
use crate::StartupStage::PreStartup;
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use crate::AlignContent::Center;
use rand::Rng;
use crate::CoreStage::First;
use crate::ui::{PokemonUI, spawn_pokemon_ui};

// reference : https://www.youtube.com/watch?v=s_4zaj8EbFI&t=757s

fn main() {
    let mut app = App::new();
    app
        .insert_resource(WindowDescriptor {
            title: "konkuRRenz".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .insert_resource(battle_plugin::generate_fighters_map())
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        // .insert_resource(WinitSettings::desktop_app())
        .insert_resource(RandomizeTimer(Timer::new(std::time::Duration::from_secs_f32(0.5), true)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(cameras_setup)
        .add_startup_system(initialize_arena)
        .add_system(randomize_arena)
        .run();
}

fn cameras_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

/// Get random fighters from HashMap, index the number of the pokemon to get it's Fighter struct.
fn spawn_random_new_fighters(fighter_map: Res<HashMap<i32, Fighter>>) -> (Fighter, Fighter) {
    let mut rng = rand::thread_rng();
    let ally_num = rng.gen_range(1..152);
    let enemy_num = rng.gen_range(1..152);

    let ally_fighter = fighter_map.get(&ally_num).cloned().unwrap().ally();
    let enemy_fighter = fighter_map.get(&enemy_num).cloned().unwrap().enemy();
    (ally_fighter, enemy_fighter)
}

/// Timer synced to the randomize_arena function, which spawns random pokemon in the every X seconds.
struct RandomizeTimer(Timer);

fn randomize_arena(mut timer: ResMut<RandomizeTimer>, query: Query<Entity, With<PokemonUI>>,
                   mut commands: Commands, asset_server: Res<AssetServer>,
                   fighter_map: Res<HashMap<i32, Fighter>>) {
    // tick timer
    timer.0.tick(Duration::from_secs_f32(0.01));

    if timer.0.finished() {
        timer.0.reset();
        // clear ui
        for e in query.iter() {
            commands.entity(e).despawn();
        }

        // get random fighter
        let (ally_fighter, enemy_fighter) = spawn_random_new_fighters(fighter_map);

        // load assets into server
        let back_id: String = format!("sprites/back_sprites/{}_back.png", ally_fighter.name.clone());
        let front_id: String = format!("sprites/front_sprites/{}_front.png", enemy_fighter.name.clone());
        let back_fighter_sprite: Handle<Image> = asset_server.load(&back_id);
        let front_fighter_sprite: Handle<Image> = asset_server.load(&front_id);

        // spawn pokemon ui
        spawn_pokemon_ui(&mut commands, &asset_server, &ally_fighter);
        spawn_pokemon_ui(&mut commands, &asset_server, &enemy_fighter);
    }
}

// Setup the arena for desired pokemon to be present. Not practical to use at the moment, included
// to show-off desired matchups.
fn initialize_arena(mut commands: Commands, asset_server: Res<AssetServer>,
                    fighter_map: Res<HashMap<i32, Fighter>>) {
    // get specific fighters
    let ally_fighter = fighter_map.get(&143).unwrap().clone().ally();
    let enemy_fighter = fighter_map.get(&1).unwrap().clone().enemy();

    // load fighter's assets
    let back_id: String = format!("sprites/back_sprites/{}_back.png", ally_fighter.name.clone());
    let front_id: String = format!("sprites/front_sprites/{}_front.png", enemy_fighter.name.clone());
    let back_fighter_sprite: Handle<Image> = asset_server.load(&back_id);
    let front_fighter_sprite: Handle<Image> = asset_server.load(&front_id);

    // spawn ui of fighters
    spawn_pokemon_ui(&mut commands, &asset_server, &ally_fighter);
    spawn_pokemon_ui(&mut commands, &asset_server, &enemy_fighter);
}