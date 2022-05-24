mod constants;

use constants::*;
use bevy::prelude::*;
use battle_plugin::*;
use battle_plugin::pokemon_roster::Pokemon;
use crate::StartupStage::PreStartup;
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};
use crate::AlignContent::Center;

// reference : https://www.youtube.com/watch?v=s_4zaj8EbFI&t=757s

fn main() {
    let mut app = App::new();
    app
        // todo: register fighter as inspectable
        // .register_inspectable::<Fighter>()
        .insert_resource(WindowDescriptor {
            title: "konkuRRenz".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<Fighter>::new())
        .add_startup_system_to_stage(PreStartup, setup_assets)
        .add_startup_system(setup_arena)
        .add_startup_system(camera_setup)
        .add_startup_system(ui_camera_setup)
        // .add_system(debug_fighters)
        .add_startup_system(debug_UI)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn ui_camera_setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn debug_fighters(query: Query<&Fighter>) {
    for fighter in query.iter() {
        println!("{:?}", fighter);
    }
}

fn debug_UI(query: Query<&Text>) {
    for text in query.iter() {
        println!("{:?}", text);
    }
}

fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // note: path starts from the /assets server
    commands.insert_resource(ArenaAssets {
        left_fighter_sprite: asset_server.load("sprites/charmeleon_sprite.png"),
        right_figher_sprite: asset_server.load("sprites/weedle_sprite.png"),
        // arena:
    });
}

// todo: think about how i want to connect all of the ui elements to the fighters
//  could append the fighter to each tag, or make a p1 and p2 component and append it likewise,
//  or ???
fn setup_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // create fighters
    let weedle = Fighter::new(
        Pokemon::Weedle, 25., 70, Allegiance::Enemy
    );
    let charmeleon = Fighter::new(
        Pokemon::Charmeleon, 25., 70, Allegiance::Ally
    );

    // spawn ui
    spawn_pokemon_ui(&mut commands, &asset_server, &weedle);
    spawn_pokemon_ui(&mut commands, &asset_server, &charmeleon);
}

fn spawn_pokemon_ui(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    // Static Assets
    spawn_pokemon_sprite(&mut commands, &asset_server, &fighter);
    spawn_name_ui(&mut commands, &asset_server, &fighter);
    // Dynamic Assets
    // . . .
}

fn spawn_name_ui(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    let mut spawn_ui = |position: Rect<Val>| {
        commands.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position,
                ..default()
            },
            // left pokemon name text
            text: Text::with_section(
                fighter.name.clone(),
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: FONT_SIZE,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        });
    };
    match fighter.allegiance.as_ref().unwrap() {
        Allegiance::Enemy => {
            let hor_adj: Val = Val::Px(43.);
            let vir_adj: Val = Val::Px(205.);
            let pos = Rect {
                right: hor_adj,
                bottom: vir_adj,
                ..default()
            };
            spawn_ui(pos)
        }
        Allegiance::Ally => {
            let hor_adj: Val = Val::Px(30.);
            let vir_adj: Val = Val::Px(5.);
            let pos = Rect {
                left: hor_adj,
                top: vir_adj,
                ..default()
            };
            spawn_ui(pos)
        }
    }
}
fn spawn_pokemon_sprite(mut commands: &mut Commands, asset_server: &Res<AssetServer>, fighter: &Fighter) {
    let id: String = format!("sprites/{}_sprite.png", fighter.name.to_lowercase().clone());
    println!("id: {}", id);
    let mut spawn_sprite = |transform: Transform| {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SPRITE_SIZE.clone())),
                ..default()
            },
            transform: transform.clone(),
            texture: asset_server.get_handle(id.as_str()).clone(),
            ..default()
        });
    };
    match fighter.allegiance.as_ref().unwrap() {
        Allegiance::Ally => {
            spawn_sprite(RIGHT_FIGHTER_TRANSFORM);
        }
        Allegiance::Enemy => {
            spawn_sprite(LEFT_FIGHTER_TRANSFORM);
        }
    }

}
