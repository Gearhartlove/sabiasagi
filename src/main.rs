mod constants;

use constants::*;
use bevy::prelude::*;
use battle_plugin::*;
use bevy_inspector_egui::{IndspectorPlugin, Inspectable};

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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_arena)
        .add_startup_system(camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new2d());
}

fn setup_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weedle:  Fighter = Fighter {
        name: "Weedle".to_string(),
        level: 6,
        hit_points: 22., //todo: change this number
    };
    let charmeleon: Fighter = Fighter {
        name: "Charmeleon".to_string(),
        level: 25,
        hit_points: 70.,
    };

    commands.insert_resource(ArenaAssets {
        left_fighter_sprite: asset_server.load("/assets/charmeleon_sprite.png"),
        right_figher_sprite:  asset_server.load("/assets/weedle_sprite.png"),
        // arena:
    });


    // create weedle fighter
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            flip_x: false,
            flip_y: false,
            anchor: Default::default(),
            ..default()
        },
        transform: Default::default(),
        ..default()
    })
        .insert(weedle);
    // create charmeleon fighter
    commands.spawn_bundle(SpriteBundle {

    })
        .insert(charmander);
}

#[derive(Inspectable, Default)]
struct Data {
    should_render: bool,
    text: String,
    #[inspectable(min = 42.0, max = 100.0)]
    size: f32,
}